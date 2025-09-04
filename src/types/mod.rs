use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AppState {
    pub show_popup: bool,
    pub tasks: Vec<String>,
    pub list_state: ListState,
    pub input_state: InputHandler,
}

#[derive(Debug, Default)]
pub struct InputHandler {
    pub input: String,
    pub character_index: usize,
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
