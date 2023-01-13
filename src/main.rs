use std::error::Error;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode};
use crossterm::{event, execute};
use crossterm::event::Event::Key;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use tui::backend::{Backend, CrosstermBackend};
use tui::{Frame, Terminal};
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph};

mod enums;
mod structs;
mod run_app;
mod user_interface;

use crate::enums::InputMode;
use crate::structs::{Password, PassMan};
use run_app::run_app;



fn main() -> Result<(), Box<dyn Error>> {
    let mut state = PassMan::new();

    enable_raw_mode();
    execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
    )?;

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_app::run_app(&mut terminal, &mut state);

    disable_raw_mode();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    );

    if let Err(e) = result {
        println!("{}", e.to_string());
    }

    Ok(())
}
