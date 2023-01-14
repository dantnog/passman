use crossterm::event::KeyCode;
use crossterm::event;
use crossterm::event::Event::Key;

use tui::backend::Backend;
use tui::Terminal;

use crate::enums::InputMode;
use crate::structs::PassMan;
use crate::user_interface::interface;


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut PassMan) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| interface(f, state))?;

        if let Key(key) = event::read()? {
            match state.mode {
                InputMode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('s') => todo!(),
                        KeyCode::Char('l') => state.change_mode(InputMode::List),
                        KeyCode::Char('h') => state.change_mode(InputMode::Help),
                        KeyCode::Char('i') => state.change_mode(InputMode::Title),
                        _ => {}
                    }
                },
                InputMode::Title => {
                    match key.code {
                        KeyCode::Esc => {
                            state.change_mode(InputMode::Normal);
                            state.clear_inputs();
                        },
                        KeyCode::Tab => state.change_mode(InputMode::Username),
                        KeyCode::Char(c) => state.new_title.push(c),
                        KeyCode::Backspace => {state.new_title.pop();}
                        _ => {}
                    }
                },
                InputMode::Username => {
                    match key.code {
                        KeyCode::Esc => {
                            state.change_mode(InputMode::Normal);
                            state.clear_inputs();
                        },
                        KeyCode::Tab => state.change_mode(InputMode::Password),
                        KeyCode::BackTab => state.change_mode(InputMode::Title),
                        KeyCode::Char(c) => state.new_username.push(c),
                        KeyCode::Backspace => {state.new_username.pop();}
                        _ => {}
                    }
                },
                InputMode::Password => {
                    match key.code {
                        KeyCode::Esc => {
                            state.change_mode(InputMode::Normal);
                            state.clear_inputs();
                        },
                        KeyCode::Tab => state.change_mode(InputMode::Submit),
                        KeyCode::BackTab => state.change_mode(InputMode::Username),
                        KeyCode::Char(c) => state.new_password.push(c),
                        KeyCode::Backspace => {state.new_password.pop();}
                        _ => {}
                    }
                },
                InputMode::List => {
                    match key.code {
                        KeyCode::Esc => state.change_mode(InputMode::Normal),
                        KeyCode::Char('l') => state.change_mode(InputMode::Normal),
                        _ => {}
                    }
                },
                InputMode::Search => {
                    match key.code {
                        KeyCode::Esc => {
                            state.change_mode(InputMode::Normal);
                            state.clear_inputs();
                        },
                        KeyCode::Tab => state.change_mode(InputMode::Username),
                        KeyCode::Char(c) => state.new_title.push(c),
                        KeyCode::Backspace => {state.new_title.pop();}
                        _ => {}
                    }
                },
                InputMode::Submit => {
                    match key.code {
                        KeyCode::Esc => {
                            state.change_mode(InputMode::Normal);
                            state.clear_inputs();
                        },
                        KeyCode::BackTab => state.change_mode(InputMode::Password),
                        KeyCode::Enter => state.save(),
                        _ => {}
                    }
                },
                InputMode::Help => {
                    match key.code {
                        KeyCode::Esc => state.change_mode(InputMode::Normal),
                        KeyCode::Char('h') => state.change_mode(InputMode::Normal),
                        _ => {}
                    }
                },
            }
        }
    }
}