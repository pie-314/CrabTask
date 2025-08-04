mod types;

use chrono::{self};
use serde_json::{from_str, to_string_pretty};
use std::fs;
use types::Todo;



use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    layout::{Constraint, Direction, Layout},
    Terminal,
    Frame,
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    execute,
    event::{self, KeyEvent, Event},
};
use std::io::{self, stdout};

fn ui<B: tui::backend::Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ].as_ref(),
        )
        .split(f.size());
    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
        .title("Block 2")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| ui(f))?;

    // Wait for a key press to exit
    loop {
        if let Event::Key(KeyEvent { .. }) = event::read()? {
            break;
        }
    }

    disable_raw_mode()?;
    // Leave alternate screen, clean up terminal
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

//main menu is in main function
fn main_menu(today: String) {
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


