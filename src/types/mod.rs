use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct AppState {
    pub show_popup: bool,
    pub tasks: Vec<String>,
    pub list_state: ListState,
}

pub struct InputHandler {
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub due_date: String,
    pub priority: String,
    pub notes: String,
}

enum InputMode {
    Normal,
    Editing,
}
