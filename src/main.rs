// assingining the task to next day

mod json_parser;
mod types;
use crate::event::KeyEventKind;
use chrono;
use chrono::{Datelike, Local, NaiveDate};
use color_eyre::eyre::{Ok, Result};
use json_parser::{json_parser, json_writer, toggle_task};
use ratatui::{
    crossterm::{
        event::{self, Event},
        terminal::enable_raw_mode,
    },
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use types::{AppState, Todo};

use crate::types::InputHandler;

impl Default for AppState {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            list_state: ListState::default(),
            show_popup: false,
            input_state: InputHandler {
                input: String::new(),
                messages: Vec::new(),
                character_index: 0,
            },
        }
    }
}

impl AppState {
    fn next(&mut self) {
        let len = self.tasks.len();
        if len == 0 {
            return; // nothing to select
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    0 // wrap to first
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        let len = self.tasks.len();
        if len == 0 {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1 // wrap to last
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}
//Main function
fn main() -> Result<()> {
    enable_raw_mode()?;
    color_eyre::install()?;
    let mut state = AppState::default();

    let today = chrono::Local::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();

    let tasks: Vec<Todo> = json_parser(today);

    let tasks: Vec<Todo> = tasks;
    state.tasks = tasks;

    state.list_state.select(Some(0)); // start from the first item

    let terminal = ratatui::init();
    let result = ui(terminal, &mut state);
    ratatui::restore();
    result
}

// Loading UI
fn ui(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if app_state.show_popup {
                        match key.code {
                            event::KeyCode::Char(c) => enter_char(&mut app_state.input_state, c),
                            event::KeyCode::Backspace => delete_char(&mut app_state.input_state),
                            event::KeyCode::Enter => {
                                app_state.show_popup = false;
                                submit_message(app_state);
                            }
                            event::KeyCode::Left => move_cursor_left(&mut app_state.input_state),
                            event::KeyCode::Right => move_cursor_right(&mut app_state.input_state),
                            event::KeyCode::Esc => app_state.show_popup = false,
                            _ => {}
                        }
                    } else {
                        match key.code {
                            event::KeyCode::Char('d') => toggle_task_done(app_state), // mark done
                            //event::KeyCode::Char('q') | event::KeyCode::Char('Q') => break,
                            event::KeyCode::Esc => break,
                            event::KeyCode::Down => app_state.next(),
                            event::KeyCode::Up => app_state.previous(),
                            event::KeyCode::Char('a') => {
                                app_state.show_popup = !app_state.show_popup
                            }
                            _ => {}
                        }
                    }
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

    let quote = Line::from("quote".bold().italic());
    Block::bordered()
        .title(quote)
        .border_type(BorderType::Rounded)
        .render(right_bottom, frame.buffer_mut());

    //Each day Task
    let today = Line::from("Tasks".bold().italic());
    let tasks_block = Block::bordered()
        .title(today)
        .border_type(BorderType::Rounded);

    let help_text = vec![Line::from(vec![
        Span::raw("Q/q"),
        Span::styled(" : quit", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::raw("Arrow Up/Down : "),
        Span::styled(
            "Moving through list",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::raw("Esc "),
        Span::styled("Exit", Style::default().add_modifier(Modifier::BOLD)),
    ])];

    let help_paragraph = Paragraph::new(help_text)
        .block(Block::default())
        .style(Style::default().add_modifier(Modifier::DIM));
    // Render full_bottom block in the bottom area
    frame.render_widget(help_paragraph, full_bottom);
    let items: Vec<ListItem> = app_state
        .tasks
        .iter()
        .map(|t| {
            if t.completed {
                ListItem::new(Span::styled(
                    &t.title,
                    Style::default().add_modifier(Modifier::CROSSED_OUT),
                ))
            } else {
                ListItem::new(Span::raw(&t.title))
            }
        })
        .collect();

    let list = List::new(items).block(tasks_block).highlight_symbol(">> "); // highlight current item
    frame.render_stateful_widget(list, left_area, &mut app_state.list_state.clone());

    //Input popup
    let input = Paragraph::new(app_state.input_state.input.as_str())
        .block(Block::bordered().title("ADD TASK"));

    if app_state.show_popup {
        let input_area = popup_area(frame.area(), 50, 10);
        frame.render_widget(input, input_area);

        #[allow(clippy::cast_possible_truncation)]
        frame.set_cursor_position(Position::new(
            input_area.x + app_state.input_state.character_index as u16 + 1,
            input_area.y + 1,
        ));
    }

    fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}

pub fn move_cursor_left(state: &mut InputHandler) {
    let cursor_moved_left = state.character_index.saturating_sub(1);
    state.character_index = clamp_cursor(state, cursor_moved_left);
}

pub fn move_cursor_right(state: &mut InputHandler) {
    let cursor_moved_right = state.character_index.saturating_add(1);
    state.character_index = clamp_cursor(state, cursor_moved_right);
}

pub fn enter_char(state: &mut InputHandler, new_char: char) {
    let index = byte_index(state);
    state.input.insert(index, new_char);
    move_cursor_right(state);
}

pub fn delete_char(state: &mut InputHandler) {
    if state.character_index > 0 {
        let current_index = state.character_index;
        let before = state.input.chars().take(current_index - 1);
        let after = state.input.chars().skip(current_index);
        state.input = before.chain(after).collect();
        move_cursor_left(state);
    }
}

pub fn submit_message(app_state: &mut AppState) {
    let input = app_state.input_state.input.clone();
    if input.trim().is_empty() {
        return;
    }

    app_state.tasks.push(Todo {
        id: uuid::Uuid::new_v4().to_string(), // or increment counter
        title: input.clone(),
        completed: false,
        due_date: chrono::Local::now().format("%Y-%m-%d").to_string(), // today’s date
    });

    let today = chrono::Local::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();
    json_writer(today, input);

    app_state.input_state.input.clear();
    app_state.input_state.character_index = 0;
}

fn byte_index(state: &InputHandler) -> usize {
    state
        .input
        .char_indices()
        .map(|(i, _)| i)
        .nth(state.character_index)
        .unwrap_or(state.input.len())
}

fn clamp_cursor(state: &InputHandler, new_pos: usize) -> usize {
    new_pos.clamp(0, state.input.chars().count())
}

pub fn toggle_task_done(app_state: &mut AppState) {
    if let Some(selected_idx) = app_state.list_state.selected() {
        // Get the selected task
        if let Some(task) = app_state.tasks.get(selected_idx) {
            let date = &task.due_date;
            let title = &task.title;

            // Call toggle_task to update the file AND in-memory task
            if let Err(e) = toggle_task(date, title) {
                eprintln!("Failed to toggle task in file: {}", e);
                return;
            }

            // Update in-memory state to match the file
            if let Some(task_mut) = app_state.tasks.get_mut(selected_idx) {
                // Set it to the same value that is in the file
                task_mut.completed = !task_mut.completed; // optional, matches file
            }
        }
    }
}
