use crate::types::Todo;
use color_eyre::eyre::Result;
use serde_json::from_str;
use std::fs;
use std::result::Result::Err;
use std::result::Result::Ok as Okk;
// JSON parsing

pub fn json_parser(date: String) -> Vec<Todo> {
    let json = std::fs::read_to_string("data_todo/todo_data.json").unwrap();
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
    let path = "data_todo/todo_data.json";

    let mut todos: Vec<Todo> = std::fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default();

    todos.push(Todo {
        id: (todos.len() + 1).to_string(),
        title: task.to_string(),
        completed: false,
        due_date: date.to_string(),
    });

    fs::write(path, serde_json::to_string_pretty(&todos).unwrap()).unwrap();
}

pub fn toggle_task(date: &str, task_title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = "data_todo/todo_data.json";

    let contents = fs::read_to_string(filename)?;
    let mut tasks: Vec<Todo> = serde_json::from_str(&contents)?;

    if let Some(task) = tasks
        .iter_mut()
        .find(|t| t.due_date == date && t.title == task_title)
    {
        task.completed = !task.completed;
    }

    let updated = serde_json::to_string_pretty(&tasks)?;
    fs::write(filename, updated)?;

    Ok(())
}

pub fn delete_task(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = "data_todo/todo_data.json";

    // 1. Read JSON file
    let contents = fs::read_to_string(filename)?;

    // 2. Parse JSON into Vec<Todo>
    let mut tasks: Vec<Todo> = serde_json::from_str(&contents)?;

    // 3. Retain everything except the matching id
    tasks.retain(|task| task.id != id);

    // 4. Write updated list back to file
    fs::write(filename, serde_json::to_string_pretty(&tasks)?)?;

    Ok(())
}
