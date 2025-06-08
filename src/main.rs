use chrono::{self};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::io;

//parsing json and returning the todays task
#[derive(Deserialize, Serialize, Debug)]
struct Todo {
    date: String,
    task: Vec<String>,
    completed: Vec<u32>,
}

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
    add_task(&today.to_string());
}

//main menu is in main function
fn main_menu() {
    let today = chrono::Utc::now().format("%d-%b-%Y");
    let val = json_parser(&today.to_string());
    //println!("{:#?}", val);
    println!("     \"{}\"", today);
    println!("         TASKS");
    //    print_todays_task(&today);
    println!("press,d to delete,e to edit,m to mark a task");
    let mut usr = String::new();
    io::stdin()
        .read_line(&mut usr)
        .expect("Please Enter a valid Choice");
    if usr.trim() == "1" {
        println!("Todays Tasks");
    //      print_todays_task(&today);
    } else if usr.trim() == "2" {
        //    check(&today);
    }
}

fn add_task(date: &str) {
    let task = String::from("eat foood");
    let json = std::fs::read_to_string("data/todo_data.json").unwrap();
    let todos = from_str::<Vec<Todo>>(&json);
    let current_val = json_parser(date);
    println!("{:#?}", current_val);

    //std:fs:write("file.json", serde_json::to_string(&my_struct).unwrap())
}

fn newday() {}
