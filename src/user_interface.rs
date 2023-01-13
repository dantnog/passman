use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph, Tabs};

use crate::enums::InputMode;
use crate::structs::PassMan;


pub fn interface<B: Backend>(f: &mut Frame<B>, state: &mut PassMan) {
    //
    //
    //  MAIN LAYOUT
    //
    //
    let parent_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(3),
            ].as_ref()
        )
        .split(f.size());

    let main_section = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(parent_chunk[1]);

    let left_section = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(3),
            ].as_ref()
        )
        .split(main_section[0]);

    let right_section = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(3),
            ].as_ref()
        )
        .split(main_section[1]);

    //
    //
    //  MENU
    //
    //
    let menu_options: Vec<&str> = vec![
        "Insert", "Search", "List", "Help", "Quit"
    ];

    let menu_spans = menu_options
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED)
                ),
                Span::styled(
                    rest,
                    Style::default()
                        .fg(Color::White)
                )
            ])
        })
        .collect();

    let menu_tabs = Tabs::new(menu_spans)
        .block(
            Block::default()
                .title("Menu")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .divider(Span::raw("|"));
    f.render_widget(menu_tabs, parent_chunk[0]);

    //
    //
    // NEW PASSWORD
    //
    //
    let left_block = Block::default()
        .title("New Password")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(left_block, left_section[0]);

    let left_block_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
        )
        .split(left_section[0]);

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
    f.render_widget(title_input, left_block_layout[0]);

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
    f.render_widget(username_input, left_block_layout[1]);
    
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
    f.render_widget(password_input, left_block_layout[2]);

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
    f.render_widget(submit_button, left_block_layout[3]);

    //
    //
    // HELP
    //
    //
    match state.mode {
        InputMode::Help => {
            let right_block = Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            f.render_widget(right_block, right_section[0]);

            let right_block_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Min(3),
                    ]
                )
                .split(right_section[0]);

            const HELP_TEXT: &str = 
r#" D              Delete
 H              Help
 I              Insert
 L              List
 S              Search
 Q              Quit (from Normal)
 Esc            Return to Normal Mode
 Tab            Next field
 Shift+Tab      Prev field
 Enter          Active button
 "#;

            let help_text = Paragraph::new(HELP_TEXT);

            f.render_widget(help_text, right_block_layout[0]);
        },
        _ => {}
    }

}