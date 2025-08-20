use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct AppState {
    pub tasks: Vec<String>,
    pub list_state: ListState,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub due_date: String, // usually "YYYY-MM-DD" format
    pub priority: String,
    pub notes: String,
}
