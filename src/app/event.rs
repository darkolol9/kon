use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::DefaultTerminal;

use crate::app::{App, AppState, Focus, Panel};
use crate::theme;

const CTRL: KeyModifiers = KeyModifiers::CONTROL;
const ALT: KeyModifiers = KeyModifiers::ALT;

pub async fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<(), String> {
    app.completion.fetch_schema(&app.db).await;

    loop {
        // Check toast expiry
        if let Some((_, time)) = &app.toast
            && time.elapsed() > Duration::from_secs(3)
        {
            app.toast = None;
        }

        terminal
            .draw(|frame| crate::tui::render(frame, &app))
            .map_err(|e| format!("Render failed: {e}"))?;

        if event::poll(Duration::from_millis(50)).map_err(|e| format!("Event poll: {e}"))?
            && let Event::Key(key) = event::read().map_err(|e| format!("Event read: {e}"))?
            && key.kind == KeyEventKind::Press
        {
            if let AppState::Executing = app.state {
                continue;
            }

            let ctrl = key.modifiers == CTRL;
            let alt = key.modifiers == ALT;

            // Global panel switching (F1/F2/F3)
            if matches!(key.code, KeyCode::F(1) | KeyCode::F(2) | KeyCode::F(3)) {
                switch_panel(&mut app, key.code);
                continue;
            }

            // Route by focus
            match app.focus {
                Focus::Input => {
                    handle_input_key(&mut app, &mut terminal, key.code, ctrl, alt).await
                }
                Focus::Results => {
                    handle_results_key(&mut app, &mut terminal, key.code, ctrl, alt).await
                }
                Focus::SchemaBrowser => handle_schema_key(&mut app, key.code).await,
                Focus::HistoryBrowser => handle_history_key(&mut app, key.code).await,
                Focus::CommandPalette => handle_palette_key(&mut app, key.code).await,
                Focus::HelpOverlay => {
                    app.help_overlay_active = false;
                    app.focus = app.prev_focus;
                }
                Focus::ConnectionsList => handle_connections_key(&mut app, key.code).await,
                Focus::ConnectionForm => handle_connection_form_key(&mut app, key.code).await,
                Focus::SettingsList => handle_settings_key(&mut app, key.code),
                Focus::DatabaseBrowser => {
                    if ctrl && matches!(key.code, KeyCode::Char('d')) {
                        app.db_browser_visible = false;
                        app.focus = Focus::Input;
                    } else {
                        handle_database_key(&mut app, key.code).await;
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

fn switch_panel(app: &mut App, code: KeyCode) {
    let new_panel = match code {
        KeyCode::F(1) => Panel::Editor,
        KeyCode::F(2) => Panel::Connections,
        KeyCode::F(3) => Panel::Settings,
        _ => return,
    };
    app.active_panel = new_panel;
    match new_panel {
        Panel::Editor => app.focus = Focus::Input,
        Panel::Connections => app.focus = Focus::ConnectionsList,
        Panel::Settings => {
            app.settings_selection = theme::ALL_THEMES
                .iter()
                .position(|t| t.name == app.theme.name)
                .unwrap_or(0);
            app.focus = Focus::SettingsList;
        }
    }
}

// --- Input focus handlers ---

async fn handle_input_key(
    app: &mut App,
    terminal: &mut DefaultTerminal,
    code: KeyCode,
    ctrl: bool,
    alt: bool,
) {
    if app.completion_active {
        match code {
            KeyCode::Tab | KeyCode::BackTab => {
                if code == KeyCode::BackTab {
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
                handle_input_raw(app, terminal, code, ctrl, alt).await;
            }
        }
    } else {
        handle_input_raw(app, terminal, code, ctrl, alt).await;
    }
}

async fn handle_input_raw(
    app: &mut App,
    terminal: &mut DefaultTerminal,
    code: KeyCode,
    ctrl: bool,
    alt: bool,
) {
    match code {
        KeyCode::Char('c') | KeyCode::Char('q') if ctrl => app.should_quit = true,
        KeyCode::Char('d') if ctrl => app.toggle_database_browser().await,
        KeyCode::Enter => app.execute_current().await,
        KeyCode::Char('o') if ctrl => {
            let _ = app.open_in_editor(terminal);
        }
        KeyCode::Char('r') if ctrl => app.refresh_schema().await,
        KeyCode::Char('v') if ctrl => app.toggle_view_mode(),
        KeyCode::Char('s') if ctrl => app.toggle_schema_browser(),
        KeyCode::Char('h') if ctrl => {
            app.history_browser_visible = !app.history_browser_visible;
            if app.history_browser_visible {
                app.history_browser_selection = 0;
                app.focus = Focus::HistoryBrowser;
            }
        }
        KeyCode::Char('p') if ctrl => app.open_command_palette(),
        KeyCode::Char('?') if ctrl => app.toggle_help(),
        KeyCode::Left if ctrl => app.scroll_x_left(),
        KeyCode::Right if ctrl => app.scroll_x_right(),
        KeyCode::Left if alt => app.focus_prev_tab(),
        KeyCode::Right if alt => app.focus_next_tab(),
        KeyCode::Tab => {
            app.completion.compute_candidates(&app.input, app.cursor);
            if app.completion.has_completions() {
                app.completion_active = true;
            }
        }
        KeyCode::Backspace => app.delete_before(),
        KeyCode::Delete => app.delete_at(),
        KeyCode::Left => app.move_left(),
        KeyCode::Right => app.move_right(),
        KeyCode::Home => app.move_home(),
        KeyCode::End => app.move_end(),
        KeyCode::Up if !app.input.is_empty() || app.history_pos.is_some() => {
            app.history_back();
        }
        KeyCode::Down if app.history_pos.is_some() => {
            app.history_forward();
        }
        KeyCode::PageUp => app.scroll_page_older(1),
        KeyCode::PageDown => app.scroll_page_newer(1),
        KeyCode::Char(c) => app.insert_char(c),
        _ => {}
    }
}

// --- Results focus ---

async fn handle_results_key(
    app: &mut App,
    terminal: &mut DefaultTerminal,
    code: KeyCode,
    ctrl: bool,
    alt: bool,
) {
    match code {
        KeyCode::PageUp => app.scroll_page_older(1),
        KeyCode::PageDown => app.scroll_page_newer(1),
        KeyCode::Char('v') if ctrl => app.toggle_view_mode(),
        KeyCode::Left if ctrl => app.scroll_x_left(),
        KeyCode::Right if ctrl => app.scroll_x_right(),
        KeyCode::Char('o') if ctrl => {
            let _ = app.open_in_editor(terminal);
        }
        KeyCode::Char('s') if ctrl => app.toggle_schema_browser(),
        KeyCode::Char('h') if ctrl => {
            app.history_browser_visible = !app.history_browser_visible;
            if app.history_browser_visible {
                app.history_browser_selection = 0;
                app.focus = Focus::HistoryBrowser;
            }
        }
        KeyCode::Char('p') if ctrl => app.open_command_palette(),
        KeyCode::Char('?') if ctrl => app.toggle_help(),
        KeyCode::Char('r') if ctrl => app.refresh_schema().await,
        KeyCode::Char('d') if ctrl => app.toggle_database_browser().await,
        KeyCode::Left if alt => app.focus_prev_tab(),
        KeyCode::Right if alt => app.focus_next_tab(),
        KeyCode::Enter | KeyCode::Char('i') => {
            app.focus = Focus::Input;
        }
        KeyCode::Esc => {
            app.focus = Focus::Input;
        }
        _ => {}
    }
}

// --- Schema browser ---

async fn handle_schema_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => {
            app.schema_browser_selection = app.schema_browser_selection.saturating_sub(1);
        }
        KeyCode::Down => {
            let max = app.completion.tables.len().saturating_sub(1);
            if app.schema_browser_selection < max {
                app.schema_browser_selection += 1;
            }
        }
        KeyCode::Enter => {
            if let Some(table) = app.completion.tables.get(app.schema_browser_selection) {
                app.input = format!("SELECT * FROM {}", table);
                app.cursor = app.input.len();
            }
            app.schema_browser_visible = false;
            app.focus = Focus::Input;
        }
        KeyCode::Esc => {
            app.schema_browser_visible = false;
            app.focus = Focus::Input;
        }
        _ => {}
    }
}

// --- Database browser ---

async fn handle_database_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => {
            app.db_browser_selection = app.db_browser_selection.saturating_sub(1);
        }
        KeyCode::Down => {
            let max = app.db_browser_databases.len().saturating_sub(1);
            if app.db_browser_selection < max {
                app.db_browser_selection += 1;
            }
        }
        KeyCode::Enter => {
            app.select_database(app.db_browser_selection).await;
        }
        KeyCode::Esc => {
            app.db_browser_visible = false;
            app.focus = Focus::Input;
        }
        _ => {}
    }
}

// --- History browser ---

async fn handle_history_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => {
            app.history_browser_selection = app.history_browser_selection.saturating_sub(1);
        }
        KeyCode::Down => {
            let max = app.history.len().saturating_sub(1);
            if app.history_browser_selection < max {
                app.history_browser_selection += 1;
            }
        }
        KeyCode::Enter => {
            let rev_idx = app.history.len().saturating_sub(1) - app.history_browser_selection;
            if let Some(sql) = app.history.get(rev_idx) {
                app.input = sql.clone();
                app.cursor = app.input.len();
            }
            app.history_browser_visible = false;
            app.focus = Focus::Input;
        }
        KeyCode::Esc => {
            app.history_browser_visible = false;
            app.focus = Focus::Input;
        }
        _ => {}
    }
}

// --- Command palette ---

async fn handle_palette_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => {
            app.command_palette_selection = app.command_palette_selection.saturating_sub(1);
        }
        KeyCode::Down => {
            let max = app.command_palette_candidates.len().saturating_sub(1);
            if app.command_palette_selection < max {
                app.command_palette_selection += 1;
            }
        }
        KeyCode::Enter => {
            app.execute_palette_selection();
        }
        KeyCode::Esc => {
            app.command_palette_active = false;
            app.focus = app.prev_focus;
        }
        KeyCode::Char(c) => {
            app.command_palette_input.push(c);
            app.command_palette_cursor += c.len_utf8();
            app.filter_command_palette();
        }
        KeyCode::Backspace => {
            if app.command_palette_cursor > 0 {
                let prev = app.command_palette_input[..app.command_palette_cursor]
                    .char_indices()
                    .next_back();
                if let Some((idx, _)) = prev {
                    app.command_palette_input
                        .drain(idx..app.command_palette_cursor);
                    app.command_palette_cursor = idx;
                }
            }
            app.filter_command_palette();
        }
        _ => {}
    }
}

// --- Connections panel ---

async fn handle_connections_key(app: &mut App, code: KeyCode) {
    if app.confirm_delete.is_some() {
        match code {
            KeyCode::Char('y') | KeyCode::Enter => {
                let idx = app.confirm_delete.unwrap();
                app.delete_connection(idx);
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                app.confirm_delete = None;
            }
            _ => {}
        }
        return;
    }

    match code {
        KeyCode::Up => {
            app.connection_selection = app.connection_selection.saturating_sub(1);
        }
        KeyCode::Down => {
            let max = app.config.connections.len().saturating_sub(1);
            if app.connection_selection < max {
                app.connection_selection += 1;
            }
        }
        KeyCode::Enter => {
            app.activate_connection(app.connection_selection);
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.start_add_connection();
        }
        KeyCode::Char('e') | KeyCode::Char('E') => {
            let list = app.config.list_connections();
            if !list.is_empty() {
                app.start_edit_connection(app.connection_selection);
            }
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if !app.config.connections.is_empty() {
                app.confirm_delete = Some(app.connection_selection);
                app.set_toast("Delete? y/n");
            }
        }
        KeyCode::Esc => {
            app.active_panel = Panel::Editor;
            app.focus = Focus::Input;
        }
        _ => {}
    }
}

async fn handle_connection_form_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Tab => app.next_connection_form_field(),
        KeyCode::BackTab => app.prev_connection_form_field(),
        KeyCode::Enter => app.submit_connection_form(),
        KeyCode::Esc => app.cancel_connection_form(),
        KeyCode::Char(c) => app.connection_form_insert(c),
        KeyCode::Backspace => app.connection_form_delete(),
        _ => {}
    }
}

// --- Settings / theme picker ---

fn handle_settings_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up => {
            app.settings_selection = app.settings_selection.saturating_sub(1);
        }
        KeyCode::Down => {
            let max = theme::ALL_THEMES.len().saturating_sub(1);
            if app.settings_selection < max {
                app.settings_selection += 1;
            }
        }
        KeyCode::Enter => {
            app.select_theme(app.settings_selection);
        }
        KeyCode::Esc => {
            app.active_panel = Panel::Editor;
            app.focus = Focus::Input;
        }
        _ => {}
    }
}
