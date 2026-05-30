use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::DefaultTerminal;
use std::io::stdout;

use crate::app::{App, AppState, Focus, QueryBlock, ViewMode};
use crate::theme;

/// Block / tab management
impl App {
    pub fn focus_next_tab(&mut self) {
        if self.query_blocks.is_empty() {
            return;
        }
        self.active_block = (self.active_block + 1).min(self.query_blocks.len() - 1);
    }

    pub fn focus_prev_tab(&mut self) {
        if self.active_block > 0 {
            self.active_block -= 1;
        }
    }

    pub fn toggle_view_mode(&mut self) {
        if self.active_block < self.view_modes.len() {
            let mode = self.view_modes[self.active_block];
            self.view_modes[self.active_block] = match mode {
                ViewMode::Table => ViewMode::Vertical,
                ViewMode::Vertical => ViewMode::Table,
            };
        }
    }

    #[allow(dead_code)]
    pub fn block_view_mode(&self, idx: usize) -> ViewMode {
        self.view_modes.get(idx).copied().unwrap_or(ViewMode::Table)
    }
}

/// SQL helpers
impl App {
    fn strip_trailing_g(sql: &str) -> (String, bool) {
        let trimmed = sql.trim_end();
        if trimmed.ends_with("\\G") || trimmed.ends_with("\\g") {
            let stripped = trimmed[..trimmed.len() - 2].trim().to_string();
            (stripped, true)
        } else {
            (sql.to_string(), false)
        }
    }
}

/// Block creation
impl App {
    fn push_block(&mut self, sql: &str, view_mode: ViewMode) -> usize {
        let idx = self.query_blocks.len();
        self.query_blocks.push(QueryBlock {
            sql: sql.to_string(),
            result: None,
            error: None,
            view_mode,
        });
        self.view_modes.push(view_mode);
        self.block_row_scroll.push(0);
        self.active_block = idx;
        self.scroll = 0;
        idx
    }
}

/// Query execution
impl App {
    pub async fn execute_current(&mut self) {
        let raw = self.input.trim().to_string();
        if raw.is_empty() {
            return;
        }

        if raw.starts_with('/') {
            self.handle_command(&raw).await;
            return;
        }

        let (sql, use_vertical) = Self::strip_trailing_g(&raw);

        self.history.push(raw);
        self.history_pos = None;
        self.input.clear();
        self.cursor = 0;

        let block = QueryBlock {
            sql: sql.clone(),
            result: None,
            error: None,
            view_mode: if use_vertical {
                ViewMode::Vertical
            } else {
                ViewMode::Table
            },
        };
        let view_mode = block.view_mode;
        self.query_blocks.push(block);
        self.view_modes.push(view_mode);
        self.block_row_scroll.push(0);
        let idx = self.query_blocks.len() - 1;
        self.active_block = idx;
        self.state = AppState::Executing;
        self.scroll = 0;

        let is_use = sql.trim_start().to_uppercase().starts_with("USE ");

        match self.db.execute(&sql).await {
            Ok(result) => {
                self.query_blocks[idx].result = Some(result);
                if is_use {
                    let db = sql
                        .trim_start()
                        .strip_prefix("USE ")
                        .or_else(|| sql.trim_start().strip_prefix("use "))
                        .map(|s| s.trim().trim_matches('`').trim_matches('\''))
                        .unwrap_or("");
                    if !db.is_empty() {
                        let base = self
                            .conn_name
                            .split(" > ")
                            .next()
                            .unwrap_or(&self.conn_name)
                            .to_string();
                        self.conn_name = format!("{} > {}", base, db);
                        self.completion.fetch_schema(&self.db).await;
                        if let Some(conn) = self.config.connections.get_mut(&base) {
                            conn.database = db.to_string();
                        }
                        let _ = self.config.save();
                    }
                }
            }
            Err(e) => {
                self.query_blocks[idx].error = Some(e);
            }
        }

        self.state = AppState::Idle;
    }
}

/// Command handling
impl App {
    async fn handle_command(&mut self, cmd: &str) {
        let trimmed = cmd.trim();
        let parts: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();
        let name = parts[0].strip_prefix('/').unwrap_or(parts[0]);
        let _args = parts.get(1).copied().unwrap_or("");

        self.input.clear();
        self.cursor = 0;

        match name {
            "theme" | "th" => self.cmd_theme(),
            "help" | "h" => self.toggle_help(),
            "clear" | "cls" => self.cmd_clear(),
            "quit" | "exit" | "q" => self.cmd_quit(),
            "tables" | "tbl" | "schemas" => self.cmd_tables().await,
            "refresh" | "rf" => self.cmd_refresh().await,
            _ => {
                self.push_block(&format!("/{}", name), ViewMode::Table);
                self.query_blocks.last_mut().unwrap().error = Some(format!(
                    "Unknown command '/{name}'. Type /help for available commands."
                ));
            }
        }
    }

    fn cmd_theme(&mut self) {
        self.settings_selection = theme::ALL_THEMES
            .iter()
            .position(|t| t.name == self.theme.name)
            .unwrap_or(0);
        self.active_panel = crate::app::state::Panel::Settings;
        self.focus = Focus::SettingsList;
    }

    fn cmd_clear(&mut self) {
        self.query_blocks.clear();
        self.view_modes.clear();
        self.block_row_scroll.clear();
        self.active_block = 0;
        self.scroll = 0;
    }

    fn cmd_quit(&mut self) {
        self.should_quit = true;
    }

    async fn cmd_tables(&mut self) {
        let idx = self.push_block("# tables", ViewMode::Table);
        match self.db.execute("SHOW TABLES").await {
            Ok(result) => self.query_blocks[idx].result = Some(result),
            Err(e) => self.query_blocks[idx].error = Some(e),
        }
    }

    async fn cmd_refresh(&mut self) {
        self.completion.fetch_schema(&self.db).await;
        self.set_toast("Schema refreshed");
    }
}

/// Schema browser
impl App {
    pub async fn refresh_schema(&mut self) {
        self.completion.fetch_schema(&self.db).await;
        self.set_toast("Schema refreshed");
    }
}

/// Database browser
impl App {
    pub async fn toggle_database_browser(&mut self) {
        if self.db_browser_visible {
            self.db_browser_visible = false;
            self.focus = self.prev_focus;
            return;
        }
        self.db_browser_visible = true;
        self.db_browser_selection = 0;
        self.db_browser_fetching = true;
        self.db_browser_error = None;
        self.prev_focus = self.focus;
        self.focus = Focus::DatabaseBrowser;

        let current_db = self.conn_name.split(" > ").nth(1).unwrap_or("").to_string();

        match self.db.fetch_databases().await {
            Ok(dbs) => {
                self.db_browser_databases = dbs;
                if let Some(pos) = self
                    .db_browser_databases
                    .iter()
                    .position(|d| d == &current_db)
                {
                    self.db_browser_selection = pos;
                }
            }
            Err(e) => {
                self.db_browser_databases = vec![];
                self.db_browser_error = Some(e);
            }
        }
        self.db_browser_fetching = false;
    }

    pub async fn select_database(&mut self, idx: usize) {
        let Some(db_name) = self.db_browser_databases.get(idx) else {
            self.db_browser_visible = false;
            self.focus = self.prev_focus;
            return;
        };
        let sql = format!("USE `{}`", db_name);
        let _ = self.db.execute(&sql).await;
        let base = self
            .conn_name
            .split(" > ")
            .next()
            .unwrap_or(&self.conn_name)
            .to_string();
        self.conn_name = format!("{} > {}", base, db_name);
        self.completion.fetch_schema(&self.db).await;
        if let Some(conn) = self.config.connections.get_mut(&base) {
            conn.database = db_name.to_string();
        }
        let _ = self.config.save();
        self.set_toast(&format!("Switched to database '{}'", db_name));
        self.db_browser_visible = false;
        self.focus = self.prev_focus;
    }
}

/// Feature toggles
impl App {
    pub fn toggle_schema_browser(&mut self) {
        if self.schema_browser_visible {
            self.schema_browser_visible = false;
            self.focus = Focus::Input;
        } else {
            self.schema_browser_visible = true;
            self.schema_browser_selection = 0;
            self.focus = Focus::SchemaBrowser;
        }
    }

    pub fn toggle_help(&mut self) {
        self.help_overlay_active = !self.help_overlay_active;
        if self.help_overlay_active {
            self.prev_focus = self.focus;
            self.focus = Focus::HelpOverlay;
        } else {
            self.focus = self.prev_focus;
        }
    }

    pub fn open_command_palette(&mut self) {
        self.command_palette_active = true;
        self.command_palette_input.clear();
        self.command_palette_cursor = 0;
        self.command_palette_candidates = self.all_palette_entries.clone();
        self.command_palette_selection = 0;
        self.prev_focus = self.focus;
        self.focus = Focus::CommandPalette;
    }

    pub fn filter_command_palette(&mut self) {
        if self.command_palette_input.is_empty() {
            self.command_palette_candidates = self.all_palette_entries.clone();
        } else {
            let query = self.command_palette_input.to_uppercase();
            self.command_palette_candidates = self
                .all_palette_entries
                .iter()
                .filter(|e| {
                    e.name.to_uppercase().contains(&query) || e.desc.to_uppercase().contains(&query)
                })
                .cloned()
                .collect();
        }
        self.command_palette_selection = 0;
    }

    pub fn execute_palette_selection(&mut self) {
        if let Some(entry) = self
            .command_palette_candidates
            .get(self.command_palette_selection)
        {
            self.command_palette_active = false;
            self.focus = self.prev_focus;
            self.input = format!("/{}", entry.name);
            self.cursor = self.input.len();
        }
    }
}

/// Open last result in external editor
impl App {
    pub fn open_in_editor(&self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        let block = self.query_blocks.last().ok_or("No query results yet")?;
        let content = format_block_as_text(block);

        let mut path = std::env::temp_dir();
        path.push(format!("kon_output_{}.txt", std::process::id()));
        std::fs::write(&path, &content).map_err(|e| format!("Failed to write temp file: {e}"))?;

        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| "vim".to_string());

        execute!(stdout(), LeaveAlternateScreen).map_err(|e| format!("Leave alt screen: {e}"))?;
        disable_raw_mode().map_err(|e| format!("Disable raw mode: {e}"))?;

        let status = std::process::Command::new(&editor)
            .arg(&path)
            .status()
            .map_err(|e| format!("Failed to launch editor '{editor}': {e}"))?;

        if !status.success() {
            eprintln!("Editor exited with non-zero status");
        }

        enable_raw_mode().map_err(|e| format!("Enable raw mode: {e}"))?;
        execute!(stdout(), EnterAlternateScreen).map_err(|e| format!("Enter alt screen: {e}"))?;
        terminal.clear().map_err(|e| format!("Clear screen: {e}"))?;

        let _ = std::fs::remove_file(&path);

        Ok(())
    }
}

fn format_block_as_text(block: &QueryBlock) -> String {
    let mut out = String::new();
    out.push_str(&format!("mysql> {}\n", block.sql));
    if let Some(result) = &block.result {
        let lines = match block.view_mode {
            ViewMode::Table => crate::tui::table::format_result_as_text(result),
            ViewMode::Vertical => crate::tui::vertical::format_vertical_as_text(result),
        };
        for line in lines {
            let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
            out.push_str(&text);
            out.push('\n');
        }
    }
    if let Some(err) = &block.error {
        out.push_str(&format!("ERROR: {err}\n"));
    }
    out.push('\n');
    out
}
