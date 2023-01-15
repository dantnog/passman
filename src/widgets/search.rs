use tui::layout::Constraint;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, BorderType, Paragraph, Row, Cell, Table};

use crate::enums::InputMode;
use crate::structs::PassMan;

pub fn new<'a>(state: &PassMan) -> (Paragraph<'a>, Table<'a>) {
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

    let rows: Vec<_> = state.search_list.iter()
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
            .title("Passwords Found")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .widths(&[
        Constraint::Percentage(30),
        Constraint::Percentage(30),
        Constraint::Percentage(40),
    ]);

    (search_input, table)
}