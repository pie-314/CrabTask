use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
    pub date: String,
    pub task: Vec<String>,
    pub completed: Vec<u32>,
}
