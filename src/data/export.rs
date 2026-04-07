use crate::data::engine::DataEngine;
use anyhow::{Context, Result};

pub fn export_csv(engine: &DataEngine, path: &str, where_clause: Option<&str>, order_by: Option<&str>) -> Result<()> {
    let mut sql = String::from("COPY (SELECT * FROM data");
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
    sql.push_str(&format!(") TO '{}' (HEADER, DELIMITER ',')", path.replace('\'', "''")));
    engine.execute_raw(&sql).context("Failed to export CSV")?;
    Ok(())
}

pub fn export_json(engine: &DataEngine, path: &str, where_clause: Option<&str>, order_by: Option<&str>) -> Result<()> {
    let mut sql = String::from("COPY (SELECT * FROM data");
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
    sql.push_str(&format!(") TO '{}' (FORMAT JSON, ARRAY true)", path.replace('\'', "''")));
    engine.execute_raw(&sql).context("Failed to export JSON")?;
    Ok(())
}

pub fn export_parquet(engine: &DataEngine, path: &str, where_clause: Option<&str>, order_by: Option<&str>) -> Result<()> {
    let mut sql = String::from("COPY (SELECT * FROM data");
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
    sql.push_str(&format!(") TO '{}' (FORMAT PARQUET)", path.replace('\'', "''")));
    engine.execute_raw(&sql).context("Failed to export Parquet")?;
    Ok(())
}
