use color_eyre::eyre::{Ok, Result};
use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    text::Line,
    widgets::{block::title, Block, BorderType, List, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::result::Result::Err;
use std::result::Result::Ok as Okk;

#[derive(Debug, Default)]
struct AppState {
    tasks: Vec<Todo>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Todo {
    date: String,
    task: String,
    is_done: bool,
}

//Main function
fn main() -> Result<()> {
    let mut state = AppState::default();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = ui(terminal, &mut state);
    ratatui::restore();
    result
}

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

// Loading UI
fn ui(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;

        //Input Handler
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

// Rendering the UI
fn render(frame: &mut Frame, app_state: &AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let title = Line::from("CrabTask".bold());
    Block::bordered()
        .title(title.centered())
        .border_type(BorderType::Rounded)
        .fg(Color::Magenta)
        .render(border_area, frame.buffer_mut());

    let [full_top, full_bottom] =
        Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)])
            .margin(0)
            .areas(border_area);

    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
            .margin(1)
            .areas(full_top);

    let [right_top, right_middle, right_bottom] = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .margin(0)
    .areas(right_area);

    //stats progress bar
    let status = Line::from("PROGRESS".bold().italic());
    Block::bordered()
        .title(status)
        .border_type(BorderType::Rounded)
        .render(right_top, frame.buffer_mut());

    // All the necessary key bindings
    let keywords = Line::from("calender".bold().italic());
    Block::bordered()
        .title(keywords)
        .border_type(BorderType::Rounded)
        .render(right_bottom, frame.buffer_mut());

    //Each day Task
    let today = Line::from("Today".bold().italic());
    Block::bordered()
        .title(today)
        .border_type(BorderType::Rounded)
        .render(left_area, frame.buffer_mut());
}
