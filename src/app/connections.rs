use crate::app::{App, ConnectionMode, Focus, Panel};
use crate::config::Connection;
use crate::db;

impl App {
    pub fn wizard_fields_in_step(step: usize) -> usize {
        match step {
            0 => 3,
            1 => 2,
            2 => 1,
            _ => 0,
        }
    }

    pub fn wizard_absolute_field(step: usize, focus: usize) -> usize {
        match step {
            0 => focus,
            1 => 3 + focus,
            2 => 5,
            _ => 0,
        }
    }

    pub fn wizard_absolute(&self) -> usize {
        Self::wizard_absolute_field(self.conn_wizard_step, self.conn_form_focus)
    }

    pub async fn connect_to_active(&mut self) -> Result<(), String> {
        let (name, conn) = self.config.active().ok_or("No active connection")?;
        let database = db::Database::connect(conn).await?;
        let header = if conn.database == "mysql" {
            name.to_string()
        } else {
            format!("{} ({})", name, conn.database)
        };
        self.db = Some(database);
        self.conn_name = header;
        self.active_panel = Panel::Editor;
        self.focus = Focus::Input;
        self.conn_mode = ConnectionMode::Browse;

        self.query_blocks.clear();
        self.active_block = 0;
        self.scroll = 0;
        self.scroll_x = 0;
        self.input.clear();
        self.cursor = 0;
        self.db_browser_databases.clear();
        self.db_browser_selection = 0;
        self.db_browser_error = None;
        self.completion.tables.clear();
        self.completion.columns.clear();

        if let Some(ref db) = self.db {
            self.completion.fetch_schema(db).await;
        }
        Ok(())
    }

    pub async fn activate_connection(&mut self, idx: usize) {
        let list = self.config.list_connections();
        if let Some((name, _)) = list.get(idx) {
            let name_str = name.to_string();
            if self.config.set_active(&name_str).is_err() {
                return;
            }
            self.db = None;
            if let Err(e) = self.connect_to_active().await {
                self.set_toast(&format!("Connection failed: {}", e));
                return;
            }
            self.set_toast(&format!("Connected to '{}'", name_str));
        }
    }

    pub async fn test_connection_at(&self, idx: usize) -> Result<(), String> {
        let list = self.config.list_connections();
        let (_, conn) = list.get(idx).ok_or("No connection selected")?;
        db::Database::connect(conn).await?;
        Ok(())
    }

    pub async fn test_connection_from_form(&self) -> Result<(), String> {
        let port: u16 = self
            .conn_form_port
            .parse()
            .map_err(|_| "Invalid port number".to_string())?;
        let conn = Connection {
            host: self.conn_form_host.clone(),
            port,
            user: self.conn_form_user.clone(),
            password: self.conn_form_password.clone(),
            database: self.conn_form_database.clone(),
        };
        db::Database::connect(&conn).await?;
        Ok(())
    }

    pub fn start_add_connection(&mut self) {
        self.conn_mode = ConnectionMode::Adding;
        self.conn_form_name.clear();
        self.conn_form_host = String::from("localhost");
        self.conn_form_port = String::from("3306");
        self.conn_form_user = String::from("root");
        self.conn_form_password.clear();
        self.conn_form_database = String::from("mysql");
        self.conn_form_focus = 0;
        self.conn_wizard_step = 0;
        self.cursor = 0;
        self.focus = Focus::ConnectionForm;
    }

    pub fn start_edit_connection(&mut self, idx: usize) {
        let list = self.config.list_connections();
        if let Some((name, conn)) = list.get(idx) {
            self.conn_mode = ConnectionMode::Editing(idx);
            self.conn_form_name = name.to_string();
            self.conn_form_host = conn.host.clone();
            self.conn_form_port = conn.port.to_string();
            self.conn_form_user = conn.user.clone();
            self.conn_form_password = conn.password.clone();
            self.conn_form_database = conn.database.clone();
            self.conn_form_focus = 0;
            self.conn_wizard_step = 0;
            self.cursor = name.len();
            self.focus = Focus::ConnectionForm;
        }
    }

    pub async fn submit_connection_form(&mut self) {
        let port: u16 = match self.conn_form_port.parse() {
            Ok(p) => p,
            Err(_) => {
                self.set_toast("Invalid port number");
                return;
            }
        };

        if self.conn_form_name.trim().is_empty() {
            self.set_toast("Name is required");
            return;
        }

        let conn = Connection {
            host: self.conn_form_host.clone(),
            port,
            user: self.conn_form_user.clone(),
            password: self.conn_form_password.clone(),
            database: self.conn_form_database.clone(),
        };

        let name = self.conn_form_name.trim().to_string();

        match self.conn_mode {
            ConnectionMode::Adding => {
                if self.config.add_connection(name.clone(), conn).is_ok() {
                    self.set_toast(&format!("Connection '{}' saved", name));
                }
            }
            ConnectionMode::Editing(_) => {
                let old_key = self
                    .config
                    .connections
                    .keys()
                    .find_map(|k| if k == &name { Some(k.clone()) } else { None });
                if let Some(key) = old_key {
                    self.config.connections.remove(&key);
                }
                if self.config.add_connection(name.clone(), conn).is_ok() {
                    self.set_toast(&format!("Connection '{}' updated", name));
                }
            }
            _ => {}
        }

        if self.db.is_none() {
            let _ = self.config.set_active(&name);
            if let Err(e) = self.connect_to_active().await {
                self.set_toast(&format!("Connection failed: {}", e));
                self.conn_mode = ConnectionMode::Browse;
                self.focus = Focus::ConnectionsList;
                return;
            }
            self.set_toast(&format!("Connected to '{}'", name));
        } else {
            self.conn_mode = ConnectionMode::Browse;
            self.focus = Focus::ConnectionsList;
        }
    }

    pub fn cancel_connection_form(&mut self) {
        self.conn_mode = ConnectionMode::Browse;
        self.focus = Focus::ConnectionsList;
    }

    pub fn delete_connection(&mut self, idx: usize) {
        let list = self.config.list_connections();
        if let Some((name, _)) = list.get(idx) {
            let name_owned = name.to_string();
            self.config.connections.remove(&name_owned);
            if self.config.active_connection.as_deref() == Some(&name_owned) {
                self.config.active_connection = None;
            }
            let _ = self.config.save();
            self.connection_selection = self
                .connection_selection
                .min(self.config.connections.len().saturating_sub(1));
            self.set_toast(&format!("Deleted '{}'", name_owned));
            self.confirm_delete = None;
        }
    }

    pub fn next_wizard_step(&mut self) {
        if self.conn_wizard_step < 2 {
            self.conn_wizard_step += 1;
            self.conn_form_focus = 0;
            self.cursor = 0;
        }
    }

    pub fn prev_wizard_step(&mut self) {
        if self.conn_wizard_step > 0 {
            self.conn_wizard_step -= 1;
            let max = Self::wizard_fields_in_step(self.conn_wizard_step);
            self.conn_form_focus = max.saturating_sub(1);
            self.cursor = 0;
        }
    }

    pub fn connection_form_insert(&mut self, c: char) {
        let abs = self.wizard_absolute();
        let field = match abs {
            0 => &mut self.conn_form_name,
            1 => &mut self.conn_form_host,
            2 => &mut self.conn_form_port,
            3 => &mut self.conn_form_user,
            4 => &mut self.conn_form_password,
            5 => &mut self.conn_form_database,
            _ => return,
        };
        field.push(c);
        self.cursor = field.len();
    }

    pub fn connection_form_delete(&mut self) {
        let abs = self.wizard_absolute();
        let field = match abs {
            0 => &mut self.conn_form_name,
            1 => &mut self.conn_form_host,
            2 => &mut self.conn_form_port,
            3 => &mut self.conn_form_user,
            4 => &mut self.conn_form_password,
            5 => &mut self.conn_form_database,
            _ => return,
        };
        if !field.is_empty() {
            field.pop();
            self.cursor = field.len();
        }
    }
}
