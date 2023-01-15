use tui::layout::Constraint;
use tui::style::{Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, BorderType, Row, Cell, Table};

use crate::structs::PassMan;

pub fn new<'a>(state: &PassMan) -> Table<'a> {
    let rows: Vec<_> = state.passwords.iter()
        .map(|item| {
            Row::new(vec![
                Cell::from(Span::raw(item.title.to_owned())),
                Cell::from(Span::raw(item.username.to_owned())),
                Cell::from(Span::raw(item.password.to_owned())),
            ])
        })
        .collect();

    let table = Table::new(rows)
    .header(
        Row::new(vec![
            Cell::from(Span::styled(
                "Title",
                Style::default().add_modifier(Modifier::BOLD)
            )),
            Cell::from(Span::styled(
                "Username",
                Style::default().add_modifier(Modifier::BOLD)
            )),
            Cell::from(Span::styled(
                "Password",
                Style::default().add_modifier(Modifier::BOLD)
            )),
        ])
    )
    .block(
        Block::default()
            .title("All Passwords")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .widths(&[
        Constraint::Percentage(30),
        Constraint::Percentage(30),
        Constraint::Percentage(40),
    ]);

    table
}