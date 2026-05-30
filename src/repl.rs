mod completion;
mod syntax;
mod ui;

use crate::db::{Database, QueryResult};
use completion::CompletionEngine;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::DefaultTerminal;
use std::io::stdout;
use std::time::Duration;

const CTRL: KeyModifiers = KeyModifiers::CONTROL;
const ALT: KeyModifiers = KeyModifiers::ALT;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Table,
    Vertical,
}

impl ViewMode {
    fn is_vertical(&self) -> bool {
        matches!(self, ViewMode::Vertical)
    }
}

pub struct App {
    pub input: String,
    pub cursor: usize,
    pub history: Vec<String>,
    pub history_pos: Option<usize>,
    pub query_blocks: Vec<QueryBlock>,
    pub db: Database,
    pub conn_name: String,
    pub state: AppState,
    pub scroll: usize,
    pub scroll_x: usize,
    pub should_quit: bool,
    pub completion: CompletionEngine,
    pub completion_active: bool,
    pub active_block: usize,
    pub view_modes: Vec<ViewMode>,
}

pub struct QueryBlock {
    pub sql: String,
    pub result: Option<QueryResult>,
    pub error: Option<String>,
    pub view_mode: ViewMode,
}

pub enum AppState {
    Idle,
    Executing,
}

impl App {
    pub fn new(db: Database, conn_name: String) -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            history: Vec::new(),
            history_pos: None,
            query_blocks: Vec::with_capacity(64),
            db,
            conn_name,
            state: AppState::Idle,
            scroll: 0,
            scroll_x: 0,
            should_quit: false,
            completion: CompletionEngine::new(),
            completion_active: false,
            active_block: 0,
            view_modes: Vec::with_capacity(64),
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    pub fn delete_before(&mut self) {
        if self.cursor > 0 {
            let prev = self.input[..self.cursor].char_indices().next_back();
            if let Some((idx, _)) = prev {
                self.input.drain(idx..self.cursor);
                self.cursor = idx;
            }
        }
    }

    pub fn delete_at(&mut self) {
        if self.cursor < self.input.len() {
            let next = self.input[self.cursor..].char_indices().nth(1);
            let end = next
                .map(|(i, _)| self.cursor + i)
                .unwrap_or(self.input.len());
            self.input.drain(self.cursor..end);
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            let prev = self.input[..self.cursor].char_indices().next_back();
            if let Some((idx, _)) = prev {
                self.cursor = idx;
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor < self.input.len() {
            let next = self.input[self.cursor..].char_indices().nth(1);
            if let Some((idx, c)) = next {
                self.cursor = self.cursor + idx + c.len_utf8();
            }
        }
    }

    pub fn move_home(&mut self) {
        self.cursor = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor = self.input.len();
    }

    pub fn history_back(&mut self) {
        if self.history.is_empty() {
            return;
        }
        match self.history_pos {
            None => {
                self.history_pos = Some(self.history.len() - 1);
                self.input = self.history[self.history.len() - 1].clone();
            }
            Some(pos) if pos > 0 => {
                self.history_pos = Some(pos - 1);
                self.input = self.history[pos - 1].clone();
            }
            _ => {}
        }
        self.cursor = self.input.len();
    }

    pub fn history_forward(&mut self) {
        match self.history_pos {
            Some(pos) if pos < self.history.len() - 1 => {
                self.history_pos = Some(pos + 1);
                self.input = self.history[pos + 1].clone();
            }
            _ => {
                self.history_pos = None;
                self.input.clear();
            }
        }
        self.cursor = self.input.len();
    }

    pub fn scroll_page_older(&mut self, page_size: usize) {
        self.scroll = self.scroll.saturating_add(page_size);
    }

    pub fn scroll_page_newer(&mut self, page_size: usize) {
        self.scroll = self.scroll.saturating_sub(page_size);
    }

    pub fn scroll_x_left(&mut self) {
        self.scroll_x = self.scroll_x.saturating_sub(4);
    }

    pub fn scroll_x_right(&mut self) {
        self.scroll_x = self.scroll_x.saturating_add(4);
    }

    fn strip_trailing_g(sql: &str) -> (String, bool) {
        let trimmed = sql.trim_end();
        if trimmed.ends_with("\\G") || trimmed.ends_with("\\g") {
            let stripped = trimmed[..trimmed.len() - 2].trim().to_string();
            (stripped, true)
        } else {
            (sql.to_string(), false)
        }
    }

    pub fn focus_next_block(&mut self) {
        if self.query_blocks.is_empty() {
            return;
        }
        self.active_block = (self.active_block + 1).min(self.query_blocks.len() - 1);
    }

    pub fn focus_prev_block(&mut self) {
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

    pub fn block_view_mode(&self, idx: usize) -> ViewMode {
        self.view_modes.get(idx).copied().unwrap_or(ViewMode::Table)
    }

    pub async fn refresh_schema(&mut self) {
        self.completion.fetch_schema(&self.db).await;
    }

    pub async fn execute_current(&mut self) {
        let raw_sql = self.input.trim().to_string();
        if raw_sql.is_empty() {
            return;
        }

        let (sql, use_vertical) = Self::strip_trailing_g(&raw_sql);

        self.history.push(raw_sql);
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
                            .unwrap_or(&self.conn_name);
                        self.conn_name = format!("{} > {}", base, db);
                        self.completion.fetch_schema(&self.db).await;
                    }
                }
            }
            Err(e) => {
                self.query_blocks[idx].error = Some(e);
            }
        }

        self.state = AppState::Idle;
    }

    pub fn open_in_editor(&self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        let block = self.query_blocks.last().ok_or("No query results yet")?;
        let content = ui::format_block_as_text(block);

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
        terminal
            .clear()
            .map_err(|e| format!("Clear screen: {e}"))?;

        let _ = std::fs::remove_file(&path);

        Ok(())
    }
}

pub async fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<(), String> {
    app.completion.fetch_schema(&app.db).await;

    loop {
        terminal
            .draw(|frame| ui::render(frame, &app))
            .map_err(|e| format!("Render failed: {e}"))?;

        if event::poll(Duration::from_millis(50)).map_err(|e| format!("Event poll failed: {e}"))?
            && let Event::Key(key) = event::read().map_err(|e| format!("Event read: {e}"))?
            && key.kind == KeyEventKind::Press
        {
            if let AppState::Executing = app.state {
                continue;
            }

            let ctrl = key.modifiers == CTRL;
            let alt = key.modifiers == ALT;

            if app.completion_active {
                match key.code {
                    KeyCode::Tab | KeyCode::BackTab => {
                        if key.code == KeyCode::BackTab {
                            app.completion.select_prev();
                        } else {
                            app.completion.select_next();
                        }
                    }
                    KeyCode::Enter => {
                        if let Some((new_input, new_cursor)) =
                            app.completion.accept_selection(&app.input, app.cursor)
                        {
                            app.input = new_input;
                            app.cursor = new_cursor;
                        }
                        app.completion_active = false;
                        app.completion.clear_candidates();
                    }
                    KeyCode::Esc => {
                        app.completion_active = false;
                        app.completion.clear_candidates();
                    }
                    KeyCode::Char(c) => {
                        app.insert_char(c);
                        app.completion.compute_candidates(&app.input, app.cursor);
                        if !app.completion.has_completions() {
                            app.completion_active = false;
                        }
                    }
                    KeyCode::Backspace => {
                        app.delete_before();
                        app.completion.compute_candidates(&app.input, app.cursor);
                        if !app.completion.has_completions() {
                            app.completion_active = false;
                        }
                    }
                    _ => {
                        app.completion_active = false;
                        app.completion.clear_candidates();
                        handle_key(&mut app, &mut terminal, key.code, ctrl, alt).await;
                    }
                }
            } else {
                handle_key(&mut app, &mut terminal, key.code, ctrl, alt).await;
            }
        }

        if app.should_quit {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

async fn handle_key(app: &mut App, terminal: &mut DefaultTerminal, code: KeyCode, ctrl: bool, alt: bool) {
    match code {
        KeyCode::Char('c') if ctrl => {
            app.should_quit = true;
        }
        KeyCode::Char('d') if ctrl => {
            app.should_quit = true;
        }
        KeyCode::Enter => {
            app.execute_current().await;
        }
        KeyCode::Char('o') if ctrl => {
            let _ = app.open_in_editor(terminal);
        }
        KeyCode::Char('r') if ctrl => {
            app.refresh_schema().await;
        }
        KeyCode::Char('v') if ctrl => {
            app.toggle_view_mode();
        }
        KeyCode::Left if ctrl => {
            app.scroll_x_left();
        }
        KeyCode::Right if ctrl => {
            app.scroll_x_right();
        }
        KeyCode::Up if alt => {
            app.focus_prev_block();
        }
        KeyCode::Down if alt => {
            app.focus_next_block();
        }
        KeyCode::Tab => {
            app.completion.compute_candidates(&app.input, app.cursor);
            if app.completion.has_completions() {
                app.completion_active = true;
            }
        }
        KeyCode::Backspace => {
            app.delete_before();
        }
        KeyCode::Delete => {
            app.delete_at();
        }
        KeyCode::Left => {
            if app.cursor > 0 {
                app.move_left();
            }
        }
        KeyCode::Right => {
            app.move_right();
        }
        KeyCode::Home => {
            app.move_home();
        }
        KeyCode::End => {
            app.move_end();
        }
        KeyCode::Up if !app.input.is_empty() || app.history_pos.is_some() => {
            app.history_back();
        }
        KeyCode::Down if app.history_pos.is_some() => {
            app.history_forward();
        }
        KeyCode::PageUp => {
            app.scroll_page_older(1);
        }
        KeyCode::PageDown => {
            app.scroll_page_newer(1);
        }
        KeyCode::Char(c) => {
            app.insert_char(c);
        }
        _ => {}
    }
}
