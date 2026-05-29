use crate::config::Connection;
use sqlx::AssertSqlSafe;
use sqlx::Column;
use sqlx::Row;
use sqlx::ValueRef;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlRow};
use sqlx::raw_sql;
use std::time::Instant;

#[allow(dead_code)]
#[derive(Debug)]
pub struct QueryResult {
    #[allow(dead_code)]
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
    pub rows_affected: u64,
    pub execution_time_ms: u128,
    pub is_query: bool,
}

pub struct Database {
    pool: MySqlPool,
}

#[allow(dead_code)]
impl Database {
    pub async fn connect(conn: &Connection) -> Result<Self, String> {
        let uri = format!(
            "mysql://{}:{}@{}:{}/{}",
            conn.user, conn.password, conn.host, conn.port, conn.database
        );
        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&uri)
            .await
            .map_err(|e| format!("Connection failed: {e}"))?;
        Ok(Self { pool })
    }

    pub async fn execute(&self, sql: &str) -> Result<QueryResult, String> {
        let start = Instant::now();
        let is_query = is_query_statement(sql);
        let sql = sql.to_string();

        if is_query {
            let rows = raw_sql(AssertSqlSafe(sql))
                .fetch_all(&self.pool)
                .await
                .map_err(|e| format!("Query failed: {e}"))?;

            let elapsed = start.elapsed().as_millis();

            let columns = if rows.is_empty() {
                vec![]
            } else {
                row_columns(&rows[0])
            };

            let data: Vec<Vec<Option<String>>> = rows.iter().map(row_values).collect();

            Ok(QueryResult {
                columns,
                rows: data,
                rows_affected: rows.len() as u64,
                execution_time_ms: elapsed,
                is_query: true,
            })
        } else {
            let result = raw_sql(AssertSqlSafe(sql))
                .execute(&self.pool)
                .await
                .map_err(|e| format!("Execute failed: {e}"))?;

            let elapsed = start.elapsed().as_millis();

            Ok(QueryResult {
                columns: vec![],
                rows: vec![],
                rows_affected: result.rows_affected(),
                execution_time_ms: elapsed,
                is_query: false,
            })
        }
    }

    pub async fn fetch_tables(&self) -> Result<Vec<String>, String> {
        let rows = raw_sql("SHOW TABLES")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to fetch tables: {e}"))?;

        let mut tables = Vec::new();
        for row in &rows {
            if let Ok(s) = row.try_get::<String, _>(0) {
                tables.push(s);
            }
        }
        Ok(tables)
    }

    pub async fn fetch_columns(&self, table: &str) -> Result<Vec<String>, String> {
        let sql = format!("SHOW COLUMNS FROM `{table}`");
        let rows = raw_sql(AssertSqlSafe(sql))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to fetch columns: {e}"))?;

        let mut columns = Vec::new();
        for row in &rows {
            if let Ok(s) = row.try_get::<String, _>(0) {
                columns.push(s);
            }
        }
        Ok(columns)
    }
}

fn row_columns(row: &MySqlRow) -> Vec<String> {
    row.columns().iter().map(|c| c.name().to_string()).collect()
}

fn row_values(row: &MySqlRow) -> Vec<Option<String>> {
    (0..row.len())
        .map(|i| -> Option<String> {
            let raw = row.try_get_raw(i);
            let raw = match raw {
                Ok(v) => v,
                Err(_) => return None,
            };
            if raw.is_null() {
                return None;
            }
            if let Ok(s) = row.try_get::<String, _>(i) {
                return Some(s);
            }
            if let Ok(n) = row.try_get::<i64, _>(i) {
                return Some(n.to_string());
            }
            if let Ok(f) = row.try_get::<f64, _>(i) {
                return Some(f.to_string());
            }
            Some("?".to_string())
        })
        .collect()
}

fn is_query_statement(sql: &str) -> bool {
    let trimmed = sql.trim();
    if trimmed.is_empty() {
        return false;
    }
    let upper = trimmed.to_uppercase();
    let starts = |s: &str| upper.starts_with(s);
    starts("SELECT") || starts("SHOW") || starts("DESCRIBE") || starts("EXPLAIN") || starts("WITH")
}
