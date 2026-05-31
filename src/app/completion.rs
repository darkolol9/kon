use std::collections::HashMap;

use crate::db::Database;
use sqlparser::dialect::MySqlDialect;
use sqlparser::tokenizer::{Token, Tokenizer};

const SQL_KEYWORDS: &[&str] = &[
    "SELECT",
    "FROM",
    "WHERE",
    "JOIN",
    "LEFT",
    "RIGHT",
    "INNER",
    "OUTER",
    "CROSS",
    "ON",
    "AND",
    "OR",
    "NOT",
    "IN",
    "EXISTS",
    "BETWEEN",
    "LIKE",
    "IS",
    "NULL",
    "AS",
    "SET",
    "VALUES",
    "INTO",
    "INSERT",
    "UPDATE",
    "DELETE",
    "CREATE",
    "ALTER",
    "DROP",
    "TABLE",
    "INDEX",
    "VIEW",
    "DATABASE",
    "USE",
    "SHOW",
    "DESCRIBE",
    "EXPLAIN",
    "ORDER",
    "BY",
    "GROUP",
    "HAVING",
    "LIMIT",
    "OFFSET",
    "UNION",
    "ALL",
    "DISTINCT",
    "CASE",
    "WHEN",
    "THEN",
    "ELSE",
    "END",
    "ASC",
    "DESC",
    "PRIMARY",
    "KEY",
    "FOREIGN",
    "REFERENCES",
    "CASCADE",
    "GRANT",
    "REVOKE",
    "COMMIT",
    "ROLLBACK",
    "BEGIN",
    "TRANSACTION",
    "TRUE",
    "FALSE",
    "WITH",
    "RECURSIVE",
    "RETURNING",
    "REPLACE",
    "TRUNCATE",
    "RENAME",
    "IF",
];

const SQL_FUNCTIONS: &[&str] = &[
    "COUNT",
    "SUM",
    "AVG",
    "MIN",
    "MAX",
    "COALESCE",
    "IFNULL",
    "NULLIF",
    "CAST",
    "CONVERT",
    "CONCAT",
    "SUBSTRING",
    "TRIM",
    "UPPER",
    "LOWER",
    "LENGTH",
    "REPLACE",
    "LOCATE",
    "NOW",
    "CURDATE",
    "CURTIME",
    "DATE",
    "YEAR",
    "MONTH",
    "DAY",
    "HOUR",
    "MINUTE",
    "SECOND",
    "DATE_FORMAT",
    "UNIX_TIMESTAMP",
    "FROM_UNIXTIME",
    "ROUND",
    "CEIL",
    "FLOOR",
    "ABS",
    "GREATEST",
    "LEAST",
    "GROUP_CONCAT",
    "JSON_EXTRACT",
    "JSON_UNQUOTE",
];

#[derive(Debug, Clone)]
pub struct CompletionCandidate {
    pub display: String,
    pub replacement: String,
    pub kind: &'static str,
    pub table: Option<String>,
}

pub struct CompletionEngine {
    pub tables: Vec<String>,
    pub columns: HashMap<String, Vec<String>>,
    candidates: Vec<CompletionCandidate>,
    selection: usize,
}

impl Default for CompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CompletionEngine {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            columns: HashMap::new(),
            candidates: Vec::new(),
            selection: 0,
        }
    }

    #[allow(dead_code)]
    pub fn current_candidate(&self) -> Option<&CompletionCandidate> {
        self.candidates.get(self.selection)
    }

    pub fn candidates(&self) -> &[CompletionCandidate] {
        &self.candidates
    }

    pub fn selection(&self) -> usize {
        self.selection
    }

    pub fn has_completions(&self) -> bool {
        !self.candidates.is_empty()
    }

    #[allow(dead_code)]
    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }

    pub fn clear_candidates(&mut self) {
        self.candidates.clear();
        self.selection = 0;
    }

    pub async fn fetch_schema(&mut self, db: &Database) {
        if let Ok(tables) = db.fetch_tables().await {
            self.tables = tables.clone();
            self.tables.sort_by_key(|a| a.to_lowercase());
            for table in &tables {
                if let Ok(cols) = db.fetch_columns(table).await {
                    self.columns.insert(table.clone(), cols);
                }
            }
        }
    }

    fn subsequence_match(pattern: &str, candidate: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }
        let pat_upper: Vec<char> = pattern.to_uppercase().chars().collect();
        let cand_upper: Vec<char> = candidate.to_uppercase().chars().collect();
        let mut pi = 0;
        for &c in &cand_upper {
            if pi < pat_upper.len() && c == pat_upper[pi] {
                pi += 1;
            }
        }
        pi == pat_upper.len()
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
        let mut result = String::with_capacity(input.len() - (end - start) + replacement.len());
        result.push_str(&input[..start]);
        result.push_str(replacement);
        result.push_str(&input[end..]);
        let new_cursor = start + replacement.len();
        (result, new_cursor)
    }

    fn determine_context(input: &str, cursor: usize) -> Context {
        let before = &input[..cursor.min(input.len())];
        if before.trim().is_empty() {
            return Context::Global;
        }

        let dialect = MySqlDialect {};
        let mut tokenizer = Tokenizer::new(&dialect, before);
        let mut tokens = Vec::new();
        let _ = tokenizer.tokenize_with_location_into_buf(&mut tokens);

        let last_keyword = tokens.iter().rev().find_map(|twl| {
            if let Token::Word(w) = &twl.token
                && w.quote_style.is_none()
                && w.keyword != sqlparser::keywords::Keyword::NoKeyword
            {
                return Some(w.value.to_uppercase());
            }
            None
        });

        match last_keyword.as_deref() {
            Some("SELECT") | Some("DISTINCT") => Context::Column,
            Some("FROM") | Some("INTO") | Some("UPDATE") | Some("TABLE") => Context::Table,
            Some("JOIN") | Some("LEFT") | Some("RIGHT") | Some("INNER") | Some("CROSS")
            | Some("OUTER") | Some("NATURAL") => Context::Table,
            Some("WHERE") | Some("AND") | Some("OR") | Some("ON") | Some("SET")
            | Some("HAVING") => Context::Column,
            Some("ORDER") | Some("GROUP") => Context::Column,
            Some("LIMIT") | Some("OFFSET") => Context::Keyword,
            Some("VALUES") | Some("AS") | Some("NOT") | Some("IS") | Some("LIKE")
            | Some("BETWEEN") | Some("EXISTS") => Context::None,
            Some("BY") => {
                let body = before
                    .trim_end()
                    .strip_suffix("BY")
                    .or_else(|| before.trim_end().strip_suffix("by"))
                    .unwrap_or(before.trim_end());
                let body_tokens = {
                    let d = MySqlDialect {};
                    let mut t = Tokenizer::new(&d, body);
                    let mut buf = Vec::new();
                    let _ = t.tokenize_with_location_into_buf(&mut buf);
                    buf
                };
                let prev = body_tokens.iter().rev().find_map(|twl| {
                    if let Token::Word(w) = &twl.token
                        && w.quote_style.is_none()
                        && w.keyword != sqlparser::keywords::Keyword::NoKeyword
                    {
                        return Some(w.value.to_uppercase());
                    }
                    None
                });
                match prev.as_deref() {
                    Some("ORDER") | Some("GROUP") => Context::Column,
                    _ => Context::Keyword,
                }
            }
            Some("INSERT") | Some("CREATE") | Some("ALTER") | Some("DROP") | Some("TRUNCATE")
            | Some("RENAME") => Context::Table,
            Some("GRANT") | Some("REVOKE") | Some("BEGIN") | Some("COMMIT") | Some("ROLLBACK") => {
                Context::None
            }
            _ => {
                let upper = before.to_uppercase();
                if upper.contains("SELECT") || upper.ends_with(',') {
                    let trimmed = before.trim_end();
                    if trimmed.ends_with(',') || upper.contains("SELECT") {
                        Context::Column
                    } else {
                        Context::Global
                    }
                } else if upper.contains("FROM") || upper.contains("JOIN") {
                    Context::Table
                } else {
                    Context::Global
                }
            }
        }
    }

    pub fn compute_candidates(&mut self, input: &str, cursor: usize) {
        self.candidates.clear();
        let word = Self::current_word(input, cursor);
        let word_upper = word.to_uppercase();

        if input.starts_with('/') {
            let word = Self::current_word(input, cursor);
            if let Some(prefix) = word.strip_prefix('/') {
                for cmd_name in crate::cmd::all_names() {
                    if Self::subsequence_match(prefix, cmd_name) {
                        self.candidates.push(CompletionCandidate {
                            display: format!("/{}", cmd_name),
                            replacement: format!("/{}", cmd_name),
                            kind: "command",
                            table: None,
                        });
                    }
                }
                self.candidates.sort_by(|a, b| a.display.cmp(&b.display));
                self.selection = 0;
            }
            return;
        }

        if let Some(dot_pos) = word.rfind('.') {
            let table_name = &word[..dot_pos];
            let col_prefix = &word[dot_pos + 1..];
            if let Some(cols) = self.columns.get(table_name) {
                for col in cols {
                    if Self::subsequence_match(col_prefix, col) {
                        self.candidates.push(CompletionCandidate {
                            display: format!("{}.{}", table_name, col),
                            replacement: format!("{}.{}", table_name, col),
                            kind: "column",
                            table: Some(table_name.to_string()),
                        });
                    }
                }
            }
            self.candidates.sort_by(|a, b| a.display.cmp(&b.display));
            self.selection = 0;
            return;
        }

        let context = Self::determine_context(input, cursor);

        let show_keywords = matches!(context, Context::Keyword | Context::Global);
        let show_tables = matches!(context, Context::Table | Context::Global);
        let show_columns = matches!(context, Context::Column | Context::Global);
        let show_functions = matches!(context, Context::Column | Context::Global);

        if show_keywords {
            for kw in SQL_KEYWORDS {
                if Self::subsequence_match(&word_upper, kw) {
                    self.candidates.push(CompletionCandidate {
                        display: kw.to_string(),
                        replacement: kw.to_string(),
                        kind: "keyword",
                        table: None,
                    });
                }
            }
        }

        if show_tables {
            for table in &self.tables {
                if Self::subsequence_match(&word_upper, table) {
                    self.candidates.push(CompletionCandidate {
                        display: table.clone(),
                        replacement: table.clone(),
                        kind: "table",
                        table: None,
                    });
                }
            }
        }

        if show_columns {
            for (table_name, cols) in &self.columns {
                for col in cols {
                    if Self::subsequence_match(&word_upper, col) {
                        self.candidates.push(CompletionCandidate {
                            display: format!("{}.{}", table_name, col),
                            replacement: col.clone(),
                            kind: "column",
                            table: Some(table_name.clone()),
                        });
                    }
                }
            }
        }

        if show_functions {
            for func in SQL_FUNCTIONS {
                if Self::subsequence_match(&word_upper, func) {
                    self.candidates.push(CompletionCandidate {
                        display: format!("{}()", func),
                        replacement: format!("{}()", func),
                        kind: "function",
                        table: None,
                    });
                }
            }
        }

        let kind_order = |k: &str| match k {
            "keyword" => 0,
            "table" => 1,
            "column" => 2,
            "function" => 3,
            _ => 4,
        };
        self.candidates.sort_by(|a, b| {
            kind_order(a.kind)
                .cmp(&kind_order(b.kind))
                .then(a.display.cmp(&b.display))
        });

        self.selection = 0;
    }

    pub fn select_next(&mut self) {
        if !self.candidates.is_empty() {
            self.selection = (self.selection + 1) % self.candidates.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.candidates.is_empty() {
            self.selection = if self.selection == 0 {
                self.candidates.len() - 1
            } else {
                self.selection - 1
            };
        }
    }

    pub fn accept_selection(&self, input: &str, cursor: usize) -> Option<(String, usize)> {
        let candidate = self.candidates.get(self.selection)?;
        Some(Self::replace_current_word(
            input,
            cursor,
            &candidate.replacement,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Context {
    Keyword,
    Table,
    Column,
    Global,
    None,
}
