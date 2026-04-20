use std::io::Write;

#[test]
fn test_group_by_executes_without_panic() {
    // Regression: execute_query used to call stmt.column_count() before the
    // statement was executed, which panics in duckdb-rs 1.10501 with
    // "The statement was not executed yet". This reproduces the same pattern
    // the engine uses now (query() first, then read column info via as_ref()).
    let conn = duckdb::Connection::open_in_memory().unwrap();
    conn.execute_batch(&format!(
        "CREATE TABLE data AS SELECT * FROM read_csv('{}', auto_detect=true, header=true)",
        "examples/employees.csv"
    ))
    .unwrap();

    let sql = r#"SELECT "name", COUNT("salary") AS COUNT_salary FROM data GROUP BY "name" ORDER BY "name""#;
    let mut stmt = conn.prepare(sql).unwrap();
    let mut rows_iter = stmt.query(duckdb::params![]).unwrap();

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

    assert_eq!(col_count, 2);
    assert_eq!(col_names[0], "name");
    assert_eq!(col_names[1], "COUNT_salary");

    let mut row_count = 0;
    while let Some(_row) = rows_iter.next().unwrap() {
        row_count += 1;
    }
    assert!(row_count > 0, "expected at least one grouped row");
}

#[test]
fn test_csv_reads_all_types() {
    // Write a test CSV
    let tmp = std::env::temp_dir().join("xeli_test.csv");
    let mut f = std::fs::File::create(&tmp).unwrap();
    writeln!(f, "id,name,salary,active,rating").unwrap();
    writeln!(f, "1,Alice,95000,true,4.5").unwrap();
    writeln!(f, "2,Bob,78000,false,3.8").unwrap();

    // Load via DuckDB
    let conn = duckdb::Connection::open_in_memory().unwrap();
    conn.execute_batch(&format!(
        "CREATE TABLE data AS SELECT * FROM read_csv('{}', auto_detect=true, header=true)",
        tmp.to_str().unwrap()
    ))
    .unwrap();

    // Query with VARCHAR casts
    let mut stmt = conn
        .prepare(
            "SELECT \"id\"::VARCHAR AS \"id\", \"name\"::VARCHAR AS \"name\", \"salary\"::VARCHAR AS \"salary\", \"active\"::VARCHAR AS \"active\", \"rating\"::VARCHAR AS \"rating\" FROM data",
        )
        .unwrap();

    let rows: Vec<Vec<String>> = stmt
        .query_map(duckdb::params![], |row| {
            Ok((0..5)
                .map(|i| {
                    row.get::<_, Option<String>>(i)
                        .unwrap_or(None)
                        .unwrap_or_else(|| "NULL".to_string())
                })
                .collect())
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0][0], "1"); // id (was NULL before)
    assert_eq!(rows[0][1], "Alice");
    assert_eq!(rows[0][2], "95000"); // salary (was NULL before)
    assert_eq!(rows[0][3], "true"); // active (was NULL before)
    assert_eq!(rows[0][4], "4.5"); // rating (was NULL before)

    println!("Row 0: {:?}", rows[0]);
    println!("Row 1: {:?}", rows[1]);
    println!("All types reading correctly!");

    std::fs::remove_file(tmp).ok();
}
