// JSON parsing
fn json_parser(date: &str) -> Option<Todo> {
    let json = std::fs::read_to_string("data/todo_data.json").unwrap();
    let todos = from_str::<Vec<Todo>>(&json);

    match todos {
        Okk(value) => {
            for daily in value {
                if daily.date == date {
                    return Some(daily);
                }
            }
            None
        }
        Err(erro) => {
            println!("Error parsing JSON: {:#?}", erro);
            None
        }
    }
}
