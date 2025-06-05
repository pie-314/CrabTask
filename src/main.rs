use chrono::{self};
use std::fs::{File, OpenOptions};
use std::io;

fn main() {
    check_for_file();
    println!("Todo APP");
    println!("1..Current Tasks");
    println!("2..Update Todo");
    println!("3..Weekly Todo");
    println!("Enter : ");
    let mut usr = String::new();
    io::stdin()
        .read_line(&mut usr)
        .expect("Please Enter a valid Choice");
}

fn check_for_file() {
    let today = chrono::Utc::now().format("%d-%b-%Y");
    let file = format!("{}.txt", today);
    let today_todos = OpenOptions::new().append(true).open(&file);
    match today_todos {
        Ok(value) => println!("pass {:?}", value),
        Err(E) => create_file(&file),
    }

    /*today_todos
    .write("aadarsh is online ".as_bytes())
    .expect("unable to write ");*/
}

fn
fn create_file(file_name: &str) {
    File::create(file_name);
}
