use crate::types::Todo;
use serde_json::from_str;
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
        Err(erro) => {
            println!("Error parsing JSON: {:#?}", erro);
            vec![]
        }
    }
}
