mod types;

use chrono::{self};
use serde_json::{from_str, to_string_pretty};
use std::fs;
use std::io;
use types::Todo;

fn json_parser(date: &str) -> Option<Todo> {
    let json = std::fs::read_to_string("data/todo_data.json").unwrap();
    let todos = from_str::<Vec<Todo>>(&json);

    match todos {
        Ok(value) => {
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

fn main() {
    let today = chrono::Utc::now().format("%d-%b-%Y");
    main_menu(today.to_string());
}

//main menu is in main function
fn main_menu(today: String) {
    //println!("{:#?}", val);
    println!("     \"{}\"", &today);
    println!("{:#?}", json_parser(&today.to_string()));
    println!("         TASKS");
    //    print_todays_task(&today);
    println!("press,d to delete,a to add,m to mark a task");
    let mut usr = String::new();
    io::stdin()
        .read_line(&mut usr)
        .expect("Please Enter a valid Choice");
    if usr.trim() == "a" {
        add_task_loop(&today);
    } else if usr.trim() == "2" {
        //    check(&today);
    }
}

//loop of add task
fn add_task_loop(date: &str) {
    loop {
        println!("\n Enter Task : ");
        let mut task = String::new();
        io::stdin()
            .read_line(&mut task)
            .expect("some error occured");
        if task.trim() == "q" {
            main();
        } else {
            add_task(&date, task);
        }
    }
}

fn add_task(date: &str, task: String) {
    let json = fs::read_to_string("data/todo_data.json").unwrap();
    let mut todos: Vec<Todo> = from_str(&json).unwrap_or_else(|_| vec![]);

    if let Some(todo) = todos.iter_mut().find(|t| t.date == date) {
        todo.task.push(task);
    } else {
        todos.push(Todo {
            date: date.to_string(),
            task: vec![task],
            completed: vec![],
        });
    }
    fs::write("data/todo_data.json", to_string_pretty(&todos).unwrap()).unwrap();
    println!("Task added successfully.");
}
