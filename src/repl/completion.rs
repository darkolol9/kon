use std::collections::HashMap;

use crate::db::Database;

const SQL_KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "JOIN", "LEFT", "RIGHT", "INNER", "OUTER", "CROSS",
    "ON", "AND", "OR", "NOT", "IN", "EXISTS", "BETWEEN", "LIKE", "IS", "NULL",
    "AS", "ON", "SET", "VALUES", "INTO", "INSERT", "UPDATE", "DELETE", "CREATE",
    "ALTER", "DROP", "TABLE", "INDEX", "VIEW", "DATABASE", "USE", "SHOW",
    "DESCRIBE", "EXPLAIN", "ORDER", "BY", "GROUP", "HAVING", "LIMIT", "OFFSET",
    "UNION", "ALL", "DISTINCT", "CASE", "WHEN", "THEN", "ELSE", "END",
    "COUNT", "SUM", "AVG", "MIN", "MAX", "COALESCE", "IFNULL", "CAST",
    "ASC", "DESC", "PRIMARY", "KEY", "FOREIGN", "REFERENCES", "CASCADE",
    "GRANT", "REVOKE", "COMMIT", "ROLLBACK", "BEGIN", "TRANSACTION",
    "TRUE", "FALSE", "WITH", "RECURSIVE", "RETURNING",
];

pub struct CompletionEngine {
    pub tables: Vec<String>,
    pub columns: HashMap<String, Vec<String>>,
    candidates: Vec<String>,
    candidate_index: usize,
}

impl CompletionEngine {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            columns: HashMap::new(),
            candidates: Vec::new(),
            candidate_index: 0,
        }
    }

    pub fn current_candidate(&self) -> Option<&str> {
        self.candidates.get(self.candidate_index).map(|s| s.as_str())
    }

    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }

    pub fn candidate_index(&self) -> usize {
        self.candidate_index
    }

    pub fn has_completions(&self) -> bool {
        !self.candidates.is_empty()
    }

    pub async fn fetch_schema(&mut self, db: &Database) {
        if let Ok(tables) = db.fetch_tables().await {
            self.tables = tables.clone();
            for table in &tables {
                if let Ok(cols) = db.fetch_columns(table).await {
                    self.columns.insert(table.clone(), cols);
                }
            }
        }
    }

    fn current_word(input: &str, cursor: usize) -> &str {
        let cursor = cursor.min(input.len());
        let start = input[..cursor]
            .rfind(|c: char| c.is_ascii_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);
        let end = input[cursor..]
            .find(|c: char| c.is_ascii_whitespace())
            .map(|i| cursor + i)
            .unwrap_or(input.len());
        &input[start..end]
    }

    fn replace_current_word(input: &str, cursor: usize, replacement: &str) -> (String, usize) {
        let cursor = cursor.min(input.len());
        let start = input[..cursor]
            .rfind(|c: char| c.is_ascii_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);
        let end = input[cursor..]
            .find(|c: char| c.is_ascii_whitespace())
            .map(|i| cursor + i)
            .unwrap_or(input.len());
        let mut result = String::with_capacity(
            input.len() - (end - start) + replacement.len(),
        );
        result.push_str(&input[..start]);
        result.push_str(replacement);
        result.push_str(&input[end..]);
        let new_cursor = start + replacement.len();
        (result, new_cursor)
    }

    pub fn complete(&mut self, input: &str, cursor: usize) -> Option<(String, usize)> {
        let word = Self::current_word(input, cursor);
        if word.is_empty() && !self.candidates.is_empty() {
            self.candidates.clear();
            return None;
        }

        let word_upper = word.to_uppercase();

        if word.is_empty() || word == self.candidates.get(self.candidate_index).map(|s| s.as_str()).unwrap_or("") {
            if self.candidates.is_empty() {
                return None;
            }
            self.candidate_index = (self.candidate_index + 1) % self.candidates.len();
            let next = &self.candidates[self.candidate_index];
            return Some(Self::replace_current_word(input, cursor, next));
        }

        self.candidates.clear();

        for kw in SQL_KEYWORDS {
            if kw.starts_with(&word_upper) && !self.candidates.contains(&kw.to_string()) {
                self.candidates.push(kw.to_string());
            }
        }

        for table in &self.tables {
            if table.to_uppercase().starts_with(&word_upper)
                && !self.candidates.contains(table)
            {
                self.candidates.push(table.clone());
            }
        }

        for cols in self.columns.values() {
            for col in cols {
                if col.to_uppercase().starts_with(&word_upper)
                    && !self.candidates.contains(col)
                {
                    self.candidates.push(col.clone());
                }
            }
        }

        self.candidates.sort();
        self.candidate_index = 0;

        if self.candidates.is_empty() {
            return None;
        }

        let next = &self.candidates[0];
        Some(Self::replace_current_word(input, cursor, next))
    }
}
