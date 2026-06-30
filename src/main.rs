mod app;
mod git;
mod my_errors;
mod ui;

use app::App;
use crossterm::event::{self, Event, KeyCode};
use my_errors::TimeMachineError;
use ratatui::{
    Frame, Terminal,
    backend::{self, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io::{self, stdout};

fn main() {
    if let Err(err) = run_app() {
        eprintln!("There wad an error while running app!");
        eprintln!("Details: {}", err);
        std::process::exit(1);
    }
}

fn run_app() -> Result<(), TimeMachineError> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;

    while !app.should_quit {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Up => app.previous_commit(),
                    KeyCode::Down => app.next_commit(),
                    _ => {}
                }
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;

    Ok(())
}
