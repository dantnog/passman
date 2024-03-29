use tui::layout::Constraint;
use tui::style::{Modifier, Style, Color};
use tui::text::Span;
use tui::widgets::{Block, Borders, BorderType, Row, Cell, Table, List, ListItem};

use crate::enums::InputMode;
use crate::structs::{PassMan, Password};

pub fn new<'a>(state: &PassMan) -> (List<'a>, Table<'a>) {
    let mut list_items: Vec<ListItem> = vec![];

    match state.mode {
        InputMode::Search => {
            list_items = state.search_list.iter()
                .map(|item| {
                    ListItem::new(
                        Span::raw(item.title.to_owned())
                    )
                })
                .collect();
        },
        _ => {
            list_items = state.passwords.iter()
                .map(|item| {
                    ListItem::new(
                        Span::raw(item.title.to_owned())
                    )
                })
                .collect();
        },
    }

    let mut selected_item: Password =  Password::new(0, "".to_string(), "".to_string(), "".to_string());
    match state.mode {
        InputMode::Search => {
            if state.search_list.len() > 0 {
                selected_item = state.search_list[state.list_state.selected().unwrap()].clone();
            }
        },
        InputMode::List => {
            if state.passwords.len() > 0 {
                selected_item = state.passwords[state.list_state.selected().unwrap()].clone();
            }
        },
        _ => {}
    }

    let list = List::new(list_items)
        .block(
            Block::default()
                .title("All Passwords")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );

    let table = Table::new(vec![Row::new(vec![
            Cell::from(Span::raw(selected_item.title.to_owned())),
            Cell::from(Span::raw(selected_item.username.to_owned())),
            Cell::from(Span::raw(selected_item.password.to_owned())),
        ])])
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
                .title("Details of Password")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ]);

    (list, table)
}