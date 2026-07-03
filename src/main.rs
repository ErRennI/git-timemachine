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
};
use std::{env, io::stdout};

use crate::git::GitManager;

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

    let arg_vector: Vec<String> = env::args().collect();

    let git_manager = if arg_vector.len() == 1 {
        GitManager::new(".")?
    } else if arg_vector.len() == 2 {
        GitManager::new(&arg_vector[1])?
    } else {
        eprintln!("Usage: git-timemachine [target-git-path]");

        return Err(TimeMachineError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid Input Count!",
        )));
    };

    let mut app = App::new(&git_manager)?;

    while !app.should_quit {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Up => app.previous_commit(&git_manager),
                    KeyCode::Down => app.next_commit(&git_manager),
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
