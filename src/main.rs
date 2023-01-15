use std::error::Error;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use tui::backend::CrosstermBackend;
use tui::Terminal;

mod enums;
mod structs;
mod run_app;
mod user_interface;
mod widgets;

use crate::structs::PassMan;
use run_app::run_app;



fn main() -> Result<(), Box<dyn Error>> {
    let mut state = PassMan::new();
    state.list_state.select(Some(0));

    enable_raw_mode();
    execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
    )?;

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut state);

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
