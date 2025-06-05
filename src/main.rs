use chrono::{self};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

fn main() {
    println!("Todo APP");
    println!("1..Current Tasks");
    println!("2..Update Todo");
    println!("3..Weekly Todo");
    println!("Enter : ");
    let mut usr = String::new();
    io::stdin()
        .read_line(&mut usr)
        .expect("Please Enter a valid Choice");
    append_data();
}

fn append_data() {
    let today = chrono::Utc::now().format("%d-%b-%Y");
    let file = format!("{}.txt", today);
    let today_todos = OpenOptions::new().open(&file);

    match &today_todos {
        Ok(_) => entry(&file),
        Err(_) => create_file(&file),
    }

    /*today_todos
    .write("aadarsh is online ".as_bytes())
    .expect("unable to write ");*/
}

fn entry(filename: &String) {
    let mut today_todos = OpenOptions::new()
        .append(true)
        .write(true)
        .open(&filename)
        .unwrap();
    loop {
        let mut entry = String::new();
        println!("Enter your task for day : ");
        io::stdin().read_line(&mut entry).expect("unable to read");
        if entry == "q" {
            main();
        } else {
            today_todos
                .write(entry.as_bytes())
                .expect("unable to write!");
        }
    }
}
fn create_file(file_name: &String) {
    File::create(file_name);
    entry(&file_name);
}
