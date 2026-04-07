use anyhow::{Context, Result};
use duckdb::{params, Connection};

pub struct DataEngine {
    conn: Connection,
}

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub columns: Vec<ColumnInfo>,
    pub rows: Vec<Vec<String>>,
    pub total_rows: usize,
}

/// Read a single cell value from a DuckDB row, handling all types.
/// Tries multiple type conversions since duckdb-rs doesn't auto-cast.
fn read_cell(row: &duckdb::Row, i: usize) -> String {
    // Try String first (VARCHAR, TEXT)
    if let Ok(val) = row.get::<_, Option<String>>(i) {
        return val.unwrap_or_else(|| "NULL".to_string());
    }
    // Try i64 (INTEGER, BIGINT, HUGEINT)
    if let Ok(val) = row.get::<_, Option<i64>>(i) {
        return val.map(|v| v.to_string()).unwrap_or_else(|| "NULL".to_string());
    }
    // Try i32 (INTEGER)
    if let Ok(val) = row.get::<_, Option<i32>>(i) {
        return val.map(|v| v.to_string()).unwrap_or_else(|| "NULL".to_string());
    }
    // Try f64 (DOUBLE, FLOAT)
    if let Ok(val) = row.get::<_, Option<f64>>(i) {
        return val
            .map(|v| {
                if v.fract() == 0.0 && v.abs() < 1e15 {
                    format!("{}", v as i64)
                } else {
                    format!("{}", v)
                }
            })
            .unwrap_or_else(|| "NULL".to_string());
    }
    // Try bool (BOOLEAN)
    if let Ok(val) = row.get::<_, Option<bool>>(i) {
        return val
            .map(|v| if v { "true" } else { "false" }.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    // Fallback
    "NULL".to_string()
}

impl DataEngine {
    pub fn new() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .context("Failed to open DuckDB in-memory database")?;
        Ok(Self { conn })
    }

    pub fn execute_raw(&self, sql: &str) -> Result<()> {
        self.conn
            .execute_batch(sql)
            .context("Failed to execute SQL")?;
        Ok(())
    }

    pub fn load_file(&self, path: &str, format: &str) -> Result<()> {
        let _ = self.conn.execute_batch("DROP TABLE IF EXISTS data");

        let sql = match format {
            "csv" | "tsv" => {
                format!(
                    "CREATE TABLE data AS SELECT * FROM read_csv('{}', auto_detect=true, header=true)",
                    path.replace('\'', "''")
                )
            }
            "json" | "jsonl" | "ndjson" => {
                format!(
                    "CREATE TABLE data AS SELECT * FROM read_json('{}', auto_detect=true)",
                    path.replace('\'', "''")
                )
            }
            "parquet" => {
                format!(
                    "CREATE TABLE data AS SELECT * FROM read_parquet('{}')",
                    path.replace('\'', "''")
                )
            }
            "xlsx" | "xls" => {
                let _ = self.conn.execute_batch("INSTALL spatial; LOAD spatial;");
                format!(
                    "CREATE TABLE data AS SELECT * FROM st_read('{}')",
                    path.replace('\'', "''")
                )
            }
            _ => {
                format!(
                    "CREATE TABLE data AS SELECT * FROM read_csv('{}', auto_detect=true)",
                    path.replace('\'', "''")
                )
            }
        };

        self.conn
            .execute_batch(&sql)
            .with_context(|| format!("Failed to load file as {}", format))?;

        Ok(())
    }

    pub fn get_schema(&self) -> Result<Vec<ColumnInfo>> {
        let mut stmt = self
            .conn
            .prepare("SELECT column_name, data_type FROM information_schema.columns WHERE table_name = 'data' ORDER BY ordinal_position")?;

        let columns = stmt
            .query_map(params![], |row| {
                Ok(ColumnInfo {
                    name: row.get(0)?,
                    data_type: row.get(1)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(columns)
    }

    pub fn get_total_rows(&self) -> Result<usize> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM data")?;
        let count: i64 = stmt.query_row(params![], |row| row.get(0))?;
        Ok(count as usize)
    }

    pub fn query_page(
        &self,
        offset: usize,
        limit: usize,
        order_by: Option<&str>,
        where_clause: Option<&str>,
    ) -> Result<QueryResult> {
        let columns = self.get_schema()?;

        // Cast all columns to VARCHAR so we get strings back reliably
        let select_cols: Vec<String> = columns
            .iter()
            .map(|c| {
                let safe = c.name.replace('"', "\"\"");
                format!("\"{}\"::VARCHAR AS \"{}\"", safe, safe)
            })
            .collect();

        let mut sql = format!("SELECT {} FROM data", select_cols.join(", "));

        if let Some(w) = where_clause {
            if !w.is_empty() {
                sql.push_str(&format!(" WHERE {}", w));
            }
        }

        if let Some(o) = order_by {
            if !o.is_empty() {
                sql.push_str(&format!(" ORDER BY {}", o));
            }
        }

        // Get total count with filters
        let count_sql = if let Some(w) = where_clause {
            if !w.is_empty() {
                format!("SELECT COUNT(*) FROM data WHERE {}", w)
            } else {
                "SELECT COUNT(*) FROM data".to_string()
            }
        } else {
            "SELECT COUNT(*) FROM data".to_string()
        };

        let mut count_stmt = self.conn.prepare(&count_sql)?;
        let total_rows: i64 = count_stmt.query_row(params![], |row| row.get(0))?;

        sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

        let mut stmt = self.conn.prepare(&sql)?;
        let col_count = columns.len();

        let rows: Vec<Vec<String>> = stmt
            .query_map(params![], |row| {
                let mut values = Vec::with_capacity(col_count);
                for i in 0..col_count {
                    values.push(read_cell(row, i));
                }
                Ok(values)
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(QueryResult {
            columns,
            rows,
            total_rows: total_rows as usize,
        })
    }

    pub fn execute_query(&self, sql: &str) -> Result<QueryResult> {
        let mut stmt = self.conn.prepare(sql)?;
        let mut rows_iter = stmt.query(params![])?;

        let (col_count, col_names): (usize, Vec<String>) = match rows_iter.as_ref() {
            Some(s) => {
                let cc = s.column_count();
                let names: Vec<String> = (0..cc)
                    .map(|i| s.column_name(i).map(|n| n.to_string()).unwrap_or_else(|_| "?".to_string()))
                    .collect();
                (cc, names)
            }
            None => (0, Vec::new()),
        };

        let mut all_rows: Vec<Vec<String>> = Vec::new();
        while let Some(row) = rows_iter.next()? {
            let mut values = Vec::with_capacity(col_count);
            for i in 0..col_count {
                values.push(read_cell(row, i));
            }
            all_rows.push(values);
        }

        let total_rows = all_rows.len();
        let columns = col_names
            .into_iter()
            .map(|name| ColumnInfo {
                name,
                data_type: "VARCHAR".to_string(),
            })
            .collect();

        Ok(QueryResult {
            columns,
            rows: all_rows,
            total_rows,
        })
    }

    pub fn get_column_stats(&self, column: &str) -> Result<Vec<(String, String)>> {
        let safe_col = format!("\"{}\"", column.replace('"', "\"\""));
        let sql = format!(
            r#"SELECT
                COUNT(*)::VARCHAR,
                COUNT({col})::VARCHAR,
                (COUNT(*) - COUNT({col}))::VARCHAR,
                COUNT(DISTINCT {col})::VARCHAR,
                MIN({col}::VARCHAR)::VARCHAR,
                MAX({col}::VARCHAR)::VARCHAR
            FROM data"#,
            col = safe_col
        );

        let mut stmt = self.conn.prepare(&sql)?;
        let row = stmt.query_row(params![], |row| {
            Ok(vec![
                ("Total".to_string(), read_cell(row, 0)),
                ("Non-null".to_string(), read_cell(row, 1)),
                ("Nulls".to_string(), read_cell(row, 2)),
                ("Unique".to_string(), read_cell(row, 3)),
                ("Min".to_string(), read_cell(row, 4)),
                ("Max".to_string(), read_cell(row, 5)),
            ])
        })?;

        // Try numeric stats
        let num_sql = format!(
            "SELECT AVG({col})::VARCHAR, MEDIAN({col})::VARCHAR, STDDEV({col})::VARCHAR FROM data WHERE TRY_CAST({col} AS DOUBLE) IS NOT NULL",
            col = safe_col
        );
        if let Ok(mut num_stmt) = self.conn.prepare(&num_sql) {
            if let Ok(num_row) = num_stmt.query_row(params![], |r| {
                Ok((
                    r.get::<_, Option<String>>(0).unwrap_or(None),
                    r.get::<_, Option<String>>(1).unwrap_or(None),
                    r.get::<_, Option<String>>(2).unwrap_or(None),
                ))
            }) {
                let mut stats = row;
                if let Some(avg) = num_row.0 {
                    stats.push(("Mean".to_string(), avg));
                }
                if let Some(med) = num_row.1 {
                    stats.push(("Median".to_string(), med));
                }
                if let Some(std) = num_row.2 {
                    stats.push(("Std Dev".to_string(), std));
                }
                return Ok(stats);
            }
        }

        Ok(row)
    }

    pub fn get_rowid(
        &self,
        display_offset: usize,
        order_by: Option<&str>,
        where_clause: Option<&str>,
    ) -> Result<i64> {
        let mut sql = "SELECT rowid FROM data".to_string();

        if let Some(w) = where_clause {
            if !w.is_empty() {
                sql.push_str(&format!(" WHERE {}", w));
            }
        }

        if let Some(o) = order_by {
            if !o.is_empty() {
                sql.push_str(&format!(" ORDER BY {}", o));
            }
        }

        sql.push_str(&format!(" LIMIT 1 OFFSET {}", display_offset));

        let mut stmt = self.conn.prepare(&sql)?;
        let rowid: i64 = stmt
            .query_row(params![], |row| row.get(0))
            .context("Failed to resolve rowid for display row")?;

        Ok(rowid)
    }

    pub fn update_cell(&self, rowid: i64, column: &str, new_value: &str) -> Result<()> {
        let safe_col = format!("\"{}\"", column.replace('"', "\"\""));
        let sql = format!(
            "UPDATE data SET {} = ? WHERE rowid = ?",
            safe_col
        );

        self.conn
            .execute(&sql, params![new_value, rowid])
            .context("Failed to update cell")?;

        Ok(())
    }

    pub fn get_histogram_data(&self, column: &str) -> Result<(Vec<(String, usize)>, f64, f64, f64)> {
        let safe_col = format!("\"{}\"", column.replace('"', "\"\""));

        // Check if column is numeric
        let check_sql = format!(
            "SELECT COUNT(*) FROM data WHERE TRY_CAST({} AS DOUBLE) IS NOT NULL",
            safe_col
        );
        let mut check_stmt = self.conn.prepare(&check_sql)?;
        let numeric_count: i64 = check_stmt.query_row(params![], |row| row.get(0))?;
        if numeric_count == 0 {
            anyhow::bail!("Column is not numeric");
        }

        // Get min, max, avg
        let stats_sql = format!(
            "SELECT MIN(TRY_CAST({col} AS DOUBLE)), MAX(TRY_CAST({col} AS DOUBLE)), AVG(TRY_CAST({col} AS DOUBLE)) FROM data WHERE TRY_CAST({col} AS DOUBLE) IS NOT NULL",
            col = safe_col
        );
        let mut stats_stmt = self.conn.prepare(&stats_sql)?;
        let (min_val, max_val, avg_val): (f64, f64, f64) = stats_stmt.query_row(params![], |row| {
            Ok((
                row.get::<_, f64>(0).unwrap_or(0.0),
                row.get::<_, f64>(1).unwrap_or(0.0),
                row.get::<_, f64>(2).unwrap_or(0.0),
            ))
        })?;

        let bins = 10usize;
        let range = max_val - min_val;

        if range == 0.0 {
            // All values are the same
            let label = format!("{:.2}", min_val);
            return Ok((vec![(label, numeric_count as usize)], min_val, max_val, avg_val));
        }

        let bin_width = range / bins as f64;

        let hist_sql = format!(
            "SELECT LEAST(FLOOR((TRY_CAST({col} AS DOUBLE) - {min}) / {bw}), {max_bin})::INTEGER AS bin, COUNT(*) AS cnt \
             FROM data WHERE TRY_CAST({col} AS DOUBLE) IS NOT NULL \
             GROUP BY bin ORDER BY bin",
            col = safe_col,
            min = min_val,
            bw = bin_width,
            max_bin = bins - 1,
        );
        let mut hist_stmt = self.conn.prepare(&hist_sql)?;
        let hist_rows: Vec<(i32, usize)> = hist_stmt
            .query_map(params![], |row| {
                Ok((
                    row.get::<_, i32>(0).unwrap_or(0),
                    row.get::<_, i64>(1).unwrap_or(0) as usize,
                ))
            })?
            .filter_map(|r| r.ok())
            .collect();

        let mut data: Vec<(String, usize)> = Vec::with_capacity(bins);
        for i in 0..bins {
            let lo = min_val + i as f64 * bin_width;
            let hi = lo + bin_width;
            let label = format!("{:.1}-{:.1}", lo, hi);
            let count = hist_rows
                .iter()
                .find(|(b, _)| *b == i as i32)
                .map(|(_, c)| *c)
                .unwrap_or(0);
            data.push((label, count));
        }

        Ok((data, min_val, max_val, avg_val))
    }

    pub fn evaluate_expression(&self, expr: &str) -> Result<QueryResult> {
        let is_aggregate = {
            let upper = expr.to_uppercase();
            upper.contains("SUM(") || upper.contains("AVG(") || upper.contains("COUNT(")
                || upper.contains("MIN(") || upper.contains("MAX(")
                || upper.contains("MEDIAN(") || upper.contains("STDDEV(")
        };

        let sql = if is_aggregate {
            format!("SELECT {} AS result FROM data", expr)
        } else {
            format!("SELECT *, ({}) AS result FROM data", expr)
        };

        self.execute_query(&sql)
    }

    pub fn add_computed_column(&self, name: &str, expr: &str) -> Result<()> {
        let safe_name = name.replace('"', "\"\"");
        let sql = format!(
            "ALTER TABLE data ADD COLUMN \"{}\" VARCHAR",
            safe_name
        );
        self.conn.execute_batch(&sql)
            .with_context(|| format!("Failed to add column '{}'", name))?;

        let update_sql = format!(
            "UPDATE data SET \"{}\" = ({})::VARCHAR",
            safe_name, expr
        );
        self.conn.execute_batch(&update_sql)
            .with_context(|| format!("Failed to compute column '{}': expression error", name))?;

        Ok(())
    }

    pub fn load_as_table(&self, path: &str, format: &str, table_name: &str) -> Result<()> {
        let _ = self.conn.execute_batch(&format!("DROP TABLE IF EXISTS {}", table_name));

        let sql = match format {
            "csv" | "tsv" => {
                format!(
                    "CREATE TABLE {} AS SELECT * FROM read_csv('{}', auto_detect=true, header=true)",
                    table_name, path.replace('\'', "''")
                )
            }
            "json" | "jsonl" | "ndjson" => {
                format!(
                    "CREATE TABLE {} AS SELECT * FROM read_json('{}', auto_detect=true)",
                    table_name, path.replace('\'', "''")
                )
            }
            "parquet" => {
                format!(
                    "CREATE TABLE {} AS SELECT * FROM read_parquet('{}')",
                    table_name, path.replace('\'', "''")
                )
            }
            "xlsx" | "xls" => {
                let _ = self.conn.execute_batch("INSTALL spatial; LOAD spatial;");
                format!(
                    "CREATE TABLE {} AS SELECT * FROM st_read('{}')",
                    table_name, path.replace('\'', "''")
                )
            }
            _ => {
                format!(
                    "CREATE TABLE {} AS SELECT * FROM read_csv('{}', auto_detect=true)",
                    table_name, path.replace('\'', "''")
                )
            }
        };

        self.conn
            .execute_batch(&sql)
            .with_context(|| format!("Failed to load '{}' as table '{}'", path, table_name))?;

        Ok(())
    }

    pub fn get_table_schema(&self, table_name: &str) -> Result<Vec<ColumnInfo>> {
        let sql = format!(
            "SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '{}' ORDER BY ordinal_position",
            table_name.replace('\'', "''")
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let columns = stmt
            .query_map(params![], |row| {
                Ok(ColumnInfo {
                    name: row.get(0)?,
                    data_type: row.get(1)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(columns)
    }

    pub fn execute_join(&self, join_type: &str, col1: &str, col2: &str) -> Result<()> {
        let safe_col1 = col1.replace('"', "\"\"");
        let safe_col2 = col2.replace('"', "\"\"");
        let sql = format!(
            "CREATE OR REPLACE TABLE data AS \
             SELECT data.*, data2.* \
             FROM data {join_type} JOIN data2 \
             ON data.\"{}\" = data2.\"{}\"",
            safe_col1, safe_col2,
            join_type = join_type,
        );
        self.conn.execute_batch(&sql)
            .context("Failed to execute join")?;

        let _ = self.conn.execute_batch("DROP TABLE IF EXISTS data2");
        Ok(())
    }

    pub fn get_sample_values(&self, column: &str, limit: usize) -> Result<Vec<String>> {
        let safe_col = format!("\"{}\"", column.replace('"', "\"\""));
        let sql = format!(
            "SELECT DISTINCT {}::VARCHAR FROM data WHERE {} IS NOT NULL LIMIT {}",
            safe_col, safe_col, limit
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let values = stmt
            .query_map(params![], |row| row.get::<_, String>(0))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(values)
    }
}
