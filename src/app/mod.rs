pub mod completion;
pub mod connections;
pub mod editor;
pub mod event;
pub mod input;
pub mod settings;
pub mod state;

pub use state::*;

use std::time::Instant;

use crate::config::{Config, Connection};
use crate::db::{Database, QueryResult};
use crate::theme;
use completion::CompletionEngine;

pub struct QueryBlock {
    pub sql: String,
    pub result: Option<QueryResult>,
    pub error: Option<String>,
    pub view_mode: ViewMode,
}

pub struct App {
    // Panel navigation
    pub active_panel: Panel,

    // Shared state
    pub db: Database,
    pub conn_name: String,
    pub theme: &'static theme::Theme,
    pub config: Config,
    pub should_quit: bool,
    pub focus: Focus,
    pub prev_focus: Focus,
    pub state: AppState,
    pub toast: Option<(String, Instant)>,

    // Editor panel
    pub input: String,
    pub cursor: usize,
    pub history: Vec<String>,
    pub history_pos: Option<usize>,
    pub query_blocks: Vec<QueryBlock>,
    pub view_modes: Vec<ViewMode>,
    pub scroll: usize,
    pub scroll_x: usize,
    pub active_block: usize,
    pub completion: CompletionEngine,
    pub completion_active: bool,
    pub schema_browser_visible: bool,
    pub schema_browser_selection: usize,
    pub history_browser_visible: bool,
    pub history_browser_selection: usize,
    pub command_palette_active: bool,
    pub command_palette_input: String,
    pub command_palette_cursor: usize,
    pub command_palette_candidates: Vec<PaletteEntry>,
    pub command_palette_selection: usize,
    pub all_palette_entries: Vec<PaletteEntry>,
    pub help_overlay_active: bool,

    // Connections panel
    pub connection_selection: usize,
    pub conn_mode: ConnectionMode,
    pub conn_form_name: String,
    pub conn_form_host: String,
    pub conn_form_port: String,
    pub conn_form_user: String,
    pub conn_form_password: String,
    pub conn_form_database: String,
    pub conn_form_focus: usize,
    pub confirm_delete: Option<usize>,

    // Settings panel
    pub settings_selection: usize,
}

impl App {
    pub fn new(db: Database, conn_name: String, theme: &'static theme::Theme) -> Self {
        let all_palette_entries = palette_entries();
        Self {
            active_panel: Panel::Editor,
            db,
            conn_name,
            theme,
            config: Config::load(),
            should_quit: false,
            focus: Focus::Input,
            prev_focus: Focus::Input,
            state: AppState::Idle,
            toast: None,

            input: String::new(),
            cursor: 0,
            history: Vec::new(),
            history_pos: None,
            query_blocks: Vec::with_capacity(64),
            view_modes: Vec::with_capacity(64),
            scroll: 0,
            scroll_x: 0,
            active_block: 0,
            completion: CompletionEngine::new(),
            completion_active: false,
            schema_browser_visible: false,
            schema_browser_selection: 0,
            history_browser_visible: false,
            history_browser_selection: 0,
            command_palette_active: false,
            command_palette_input: String::new(),
            command_palette_cursor: 0,
            command_palette_candidates: Vec::new(),
            command_palette_selection: 0,
            all_palette_entries,
            help_overlay_active: false,

            connection_selection: 0,
            conn_mode: ConnectionMode::Browse,
            conn_form_name: String::new(),
            conn_form_host: String::from("localhost"),
            conn_form_port: String::from("3306"),
            conn_form_user: String::from("root"),
            conn_form_password: String::new(),
            conn_form_database: String::from("mysql"),
            conn_form_focus: 0,
            confirm_delete: None,

            settings_selection: 0,
        }
    }

    #[allow(dead_code)]
    pub fn connections_list(&self) -> Vec<(&str, &Connection)> {
        self.config.list_connections()
    }

    pub fn set_toast(&mut self, msg: &str) {
        self.toast = Some((msg.to_string(), Instant::now()));
    }
}
