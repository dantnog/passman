use tui::layout::Constraint;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, BorderType, Paragraph, Row, Cell, Table};

use crate::enums::InputMode;
use crate::structs::PassMan;

pub fn new<'a>(state: &PassMan) -> Paragraph<'a> {
    let search_input = Paragraph::new(state.search_text.to_owned())
        .block(
            Block::default()
                .title("Search")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style(
            match state.mode {
                InputMode::Search => Style::default()
                    .fg(Color::Yellow),
                _ => Style::default()
            }
        );

    search_input
}