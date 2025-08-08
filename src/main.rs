use color_eyre::eyre::{Ok, Result};
use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::title, calendar::CalendarEventStore, Block, BorderType, Borders, List, Paragraph,
        Widget,
    },
    DefaultTerminal, Frame,
};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::result::Result::Err;
use std::result::Result::Ok as Okk;

use chrono::{Datelike, Local, NaiveDate, Weekday};

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

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

//calendar
fn generate_calendar_lines() -> Vec<Line<'static>> {
    let today = Local::now().date_naive();
    let (year, month, day) = (today.year(), today.month(), today.day());

    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let start_weekday = first_day.weekday().num_days_from_monday(); // 0 = Mon

    // Days in month
    let days_in_month = match month {
        1 => 31,
        2 if is_leap_year(year) => 29,
        2 => 28,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30,
    };

    let mut lines = Vec::new();

    // Month header
    lines.push(Line::from(format!("{} {}", month_name(month), year)));
    lines.push(Line::from("Mo Tu We Th Fr Sa Su"));

    let mut week = Vec::new();

    // Empty cells before first day
    for _ in 0..start_weekday {
        week.push(Span::raw("   "));
    }

    // Fill days
    for d in 1..=days_in_month {
        if d == day {
            week.push(Span::styled(
                format!("{:>2} ", d),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ));
        } else {
            week.push(Span::raw(format!("{:>2} ", d)));
        }

        if (d + start_weekday as u32) % 7 == 0 {
            lines.push(Line::from(week.clone()));
            week.clear();
        }
    }

    if !week.is_empty() {
        lines.push(Line::from(week));
    }

    lines
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
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
        Layout::horizontal([Constraint::Percentage(82), Constraint::Percentage(18)])
            .margin(1)
            .areas(full_top);

    let [right_top, right_middle, right_bottom] = Layout::vertical([
        Constraint::Percentage(20),
        Constraint::Percentage(40),
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

    let calendar_lines = generate_calendar_lines();
    let calendar_widget = Paragraph::new(calendar_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(calendar_widget, right_middle);

    //Each day Task
    let today = Line::from("Tasks".bold().italic());
    Block::bordered()
        .title(today)
        .border_type(BorderType::Rounded)
        .render(left_area, frame.buffer_mut());
}
