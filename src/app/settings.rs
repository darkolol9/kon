use crate::app::App;
use crate::config::Config;
use crate::theme;

impl App {
    pub fn set_theme(&mut self, name: &str) {
        if let Some(t) = theme::from_name(name) {
            self.theme = t;
            let mut config = Config::load();
            config.theme = Some(name.to_string());
            let _ = config.save();
        }
    }

    pub fn select_theme(&mut self, idx: usize) {
        if let Some(t) = theme::ALL_THEMES.get(idx) {
            self.set_theme(t.name);
            self.set_toast(&format!("Theme: {}", t.name));
        }
    }
}
