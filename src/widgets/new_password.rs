use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType, Paragraph};

use crate::enums::InputMode;
use crate::structs::PassMan;

pub fn new(state: &PassMan) -> (Paragraph, Paragraph, Paragraph, Paragraph) {
    let title_input = Paragraph::new(state.new_title.to_owned())
        .block(
            Block::default()
                .title("Title")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style(
            match state.mode {
                InputMode::Title => Style::default()
                    .fg(Color::Yellow),
                _ => Style::default()
            }
        );

    let username_input = Paragraph::new(state.new_username.to_owned())
        .block(
            Block::default()
                .title("Username")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style(
            match state.mode {
                InputMode::Username => Style::default()
                    .fg(Color::Yellow),
                _ => Style::default()
            }
        );

    let password_input = Paragraph::new(state.new_password.to_owned())
        .block(
            Block::default()
                .title("Password")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style(
            match state.mode {
                InputMode::Password => Style::default()
                    .fg(Color::Yellow),
                _ => Style::default()
            }
        );

    let submit_button = Paragraph::new("Save")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style(
            match state.mode {
                InputMode::Submit => Style::default()
                    .fg(Color::Yellow),
                _ => Style::default()
            }
        );

    (title_input, username_input, password_input, submit_button)
}