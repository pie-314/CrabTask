use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{self};
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::Write;

//main menu is in main function
fn main() {
    let today = chrono::Utc::now().format("%d-%b-%Y");
    println!("Todo APP");
    println!("1..Current Tasks");
    println!("2..Update Todo");
    println!("3..Weekly Todo");
    println!("Enter : ");
    let mut usr = String::new();
    io::stdin()
        .read_line(&mut usr)
        .expect("Please Enter a valid Choice");
    if usr.trim() == "1" {
        println!("Todays Tasks");
        print_todays_task(&today);
    } else if usr.trim() == "2" {
        check(&today);
    }
}

//if file is present calls entry, if not runs create_file
fn check(today: &DelayedFormat<StrftimeItems>) {
    let file = format!("{}.txt", &today);
    let today_todos = OpenOptions::new().open(&file);

    match &today_todos {
        Ok(_) => entry(&file),
        Err(_) => create_file(&file),
    }
}
//prints todays current tasks
fn print_todays_task(today: &DelayedFormat<StrftimeItems<'_>>) {
    let file = format!("{}.txt", &today);
    let contents = fs::read_to_string(&file);
    println!("\nTasks for today are ");
    match contents {
        Ok(_) => {
            for i in contents {
                println!("{}", i);
            }
        }
        Err(values) => {
            println!("{}", values);
            create_file(&file);
            main();
        }
    }
}
//Appends data to file
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
        if entry.trim() == String::from("q") {
            main();
        } else {
            today_todos
                .write(entry.as_bytes())
                .expect("unable to write!");
        }
    }
}

//creates file
fn create_file(file_name: &String) {
    let filestatus = File::create(file_name);
    match filestatus {
        Ok(_) => entry(&file_name),
        Err(_) => println!("Unable to create file"),
    }
}
