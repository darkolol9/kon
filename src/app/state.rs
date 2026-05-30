use crate::cmd;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Editor,
    Connections,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ViewMode {
    #[default]
    Table,
    Vertical,
}

impl ViewMode {
    #[allow(dead_code)]
    pub fn is_vertical(&self) -> bool {
        matches!(self, ViewMode::Vertical)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Idle,
    Executing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Input,
    #[allow(dead_code)]
    Results,
    SchemaBrowser,
    HistoryBrowser,
    CommandPalette,
    HelpOverlay,
    ConnectionsList,
    ConnectionForm,
    SettingsList,
    DatabaseBrowser,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionMode {
    Browse,
    Adding,
    Editing(usize),
}

#[derive(Clone)]
pub struct PaletteEntry {
    pub name: &'static str,
    pub desc: &'static str,
}

pub fn palette_entries() -> Vec<PaletteEntry> {
    cmd::COMMANDS
        .iter()
        .map(|c| PaletteEntry {
            name: c.name,
            desc: c.description,
        })
        .collect()
}
