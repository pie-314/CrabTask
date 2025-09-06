use crate::types::Todo;
use serde_json::from_str;
use std::fs;
use std::result::Result::Err;
use std::result::Result::Ok as Okk;
// JSON parsing

pub fn json_parser(date: String) -> Vec<Todo> {
    let json = std::fs::read_to_string("data/todo_data.json").unwrap();
    let todos = from_str::<Vec<Todo>>(&json);

    match todos {
        Okk(value) => {
            let matched: Vec<Todo> = value
                .into_iter()
                .filter(|daily| daily.due_date == date)
                .collect();
            matched
        }
        Err(err) => {
            println!("Error parsing JSON: {:#?}", err);
            vec![]
        }
    }
}

pub fn json_writer(date: String, task: String) {
    let path = "data/todo_data.json";

    let mut todos: Vec<Todo> = std::fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default();

    // Add new todo
    todos.push(Todo {
        id: (todos.len() + 1).to_string(),
        title: task.to_string(),
        completed: false,
        due_date: date.to_string(),
    });

    // Write back to file
    fs::write(path, serde_json::to_string_pretty(&todos).unwrap()).unwrap();
}
