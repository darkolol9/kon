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
    pub should_quit: bool,
    pub completion: CompletionEngine,
}

pub struct QueryBlock {
    pub sql: String,
    pub result: Option<QueryResult>,
    pub error: Option<String>,
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
            should_quit: false,
            completion: CompletionEngine::new(),
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

    pub async fn execute_current(&mut self) {
        let sql = self.input.trim().to_string();
        if sql.is_empty() {
            return;
        }

        self.history.push(sql.clone());
        self.history_pos = None;
        self.input.clear();
        self.cursor = 0;

        let block = QueryBlock {
            sql: sql.clone(),
            result: None,
            error: None,
        };
        self.query_blocks.push(block);
        let idx = self.query_blocks.len() - 1;
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

    pub fn open_in_editor(&self) -> Result<(), String> {
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

        let _ = std::fs::remove_file(&path);

        Ok(())
    }

    pub fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll < usize::MAX {
            self.scroll += 1;
        }
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
            match key.code {
                KeyCode::Char('c') if key.modifiers == CTRL => {
                    app.should_quit = true;
                }
                KeyCode::Char('d') if key.modifiers == CTRL => {
                    app.should_quit = true;
                }
                KeyCode::Enter => {
                    app.execute_current().await;
                }
                KeyCode::Char('o') if key.modifiers == CTRL => {
                    let _ = app.open_in_editor();
                    let _ = terminal.clear();
                }
                KeyCode::Backspace => {
                    app.delete_before();
                }
                KeyCode::Delete => {
                    app.delete_at();
                }
                KeyCode::Left => {
                    app.move_left();
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
                KeyCode::Tab => {
                    if let Some((new_input, new_cursor)) =
                        app.completion.complete(&app.input, app.cursor)
                    {
                        app.input = new_input;
                        app.cursor = new_cursor;
                    }
                }
                KeyCode::PageUp => {
                    app.scroll_down();
                }
                KeyCode::PageDown => {
                    app.scroll_up();
                }
                KeyCode::Char(c) => {
                    app.insert_char(c);
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}
