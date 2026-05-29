use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub active_connection: Option<String>,
    pub connections: HashMap<String, Connection>,
}

impl Config {
    fn path() -> PathBuf {
        let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
        base.join("kon").join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::path();
        if !path.exists() {
            return Config::default();
        }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return Config::default(),
        };
        toml::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create config dir: {e}"))?;
        }
        let content =
            toml::to_string_pretty(self).map_err(|e| format!("Failed to serialize config: {e}"))?;
        fs::write(&path, content).map_err(|e| format!("Failed to write config: {e}"))?;
        Ok(())
    }

    pub fn list_connections(&self) -> Vec<(&str, &Connection)> {
        let mut names: Vec<&str> = self.connections.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
            .into_iter()
            .map(|n| (n, &self.connections[n]))
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_connection(&self, name: &str) -> Option<&Connection> {
        self.connections.get(name)
    }

    pub fn set_active(&mut self, name: &str) -> Result<(), String> {
        if !self.connections.contains_key(name) {
            return Err(format!("Connection '{}' not found", name));
        }
        self.active_connection = Some(name.to_string());
        self.save()
    }

    pub fn add_connection(&mut self, name: String, conn: Connection) -> Result<(), String> {
        self.connections.insert(name, conn);
        self.save()
    }

    pub fn wildcard_match<'a>(&'a self, pattern: &str) -> Vec<&'a str> {
        let pattern_lower = pattern.to_lowercase();
        let has_wild = pattern_lower.contains('*') || pattern_lower.contains('?');
        self.connections
            .keys()
            .filter(|name| {
                let name_lower = name.to_lowercase();
                if has_wild {
                    wildcard_match(&name_lower, &pattern_lower)
                } else {
                    name_lower.contains(&pattern_lower)
                }
            })
            .map(|s| s.as_str())
            .collect()
    }
}

fn wildcard_match(name: &str, pattern: &str) -> bool {
    let name_chars: Vec<char> = name.chars().collect();
    let pat_chars: Vec<char> = pattern.chars().collect();
    let (mut ni, mut pi) = (0, 0);
    let (mut star_n, mut star_p): (Option<usize>, Option<usize>) = (None, None);
    while ni < name_chars.len() {
        if pi < pat_chars.len() && (pat_chars[pi] == name_chars[ni] || pat_chars[pi] == '?') {
            ni += 1;
            pi += 1;
        } else if pi < pat_chars.len() && pat_chars[pi] == '*' {
            star_n = Some(ni);
            star_p = Some(pi);
            pi += 1;
        } else if let (Some(sn), Some(sp)) = (star_n, star_p) {
            ni = sn + 1;
            star_n = Some(ni);
            pi = sp + 1;
        } else {
            return false;
        }
    }
    while pi < pat_chars.len() && pat_chars[pi] == '*' {
        pi += 1;
    }
    pi == pat_chars.len()
}

impl Config {
    pub fn active(&self) -> Option<(&str, &Connection)> {
        self.active_connection
            .as_ref()
            .and_then(|name| self.connections.get(name).map(|c| (name.as_str(), c)))
    }
}
