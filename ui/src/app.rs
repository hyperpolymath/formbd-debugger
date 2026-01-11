// SPDX-License-Identifier: AGPL-3.0-or-later
//! Application state management

/// Current view in the debugger
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Home,
    Schema,
    Timeline,
    Diagnose,
    Recover,
    Help,
}

/// Application state
pub struct App {
    pub running: bool,
    pub view: View,
    pub connected: bool,
    pub connection_string: Option<String>,
    pub db_type: String,
    pub status_message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            view: View::Home,
            connected: false,
            connection_string: None,
            db_type: "formdb".to_string(),
            status_message: "Ready".to_string(),
        }
    }
}

impl App {
    /// Create a new app instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle keyboard input
    pub fn on_key(&mut self, key: char) {
        match key {
            'q' => self.running = false,
            'h' => self.view = View::Home,
            's' => self.view = View::Schema,
            't' => self.view = View::Timeline,
            'd' => self.view = View::Diagnose,
            'r' => self.view = View::Recover,
            '?' => self.view = View::Help,
            _ => {}
        }
    }
}
