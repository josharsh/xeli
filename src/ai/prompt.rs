pub fn build_prompt(query: &str, schema: &[String], sample_values: &[String]) -> String {
    let schema_str = schema.join("\n  - ");
    let samples_str = sample_values.join("\n  - ");

    format!(
        r#"You are a SQL query generator for DuckDB. Convert the user's natural language question into a valid DuckDB SQL query.

Rules:
- Output ONLY the SQL query. No markdown, no explanation, no code fences.
- The table is named "data".
- Use DuckDB SQL dialect (supports ILIKE, regexp_matches, LIST, STRUCT, etc.).
- Always quote column names with double quotes if they contain spaces or special characters.
- For string matching, prefer ILIKE for case-insensitive matching.
- If the user asks for "top N", use LIMIT N with appropriate ORDER BY.
- If the user asks to "group by" or "aggregate", use GROUP BY with appropriate aggregate functions.
- Return all columns with SELECT * unless the user specifies particular columns.

Table schema:
  - {schema_str}

Sample values:
  - {samples_str}

User question: {query}

SQL:"#
    )
}
