use color_eyre::{
    eyre::{Ok, Result},
    owo_colors::OwoColorize,
};
use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    text::Line,
    widgets::{block::title, Block, BorderType, List, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = ui(terminal, &mut state);
    ratatui::restore();
    result
}

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

fn render(frame: &mut Frame, app_state: &AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let title = Line::from("CrabTask".bold());
    Block::bordered()
        .title(title.centered())
        .border_type(BorderType::Thick)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());
    //List::new(app_state::items).render(area, buf);

    //Paragraph::new("Hello from the application").render(frame.area(), frame.buffer_mut());
}
