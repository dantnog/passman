use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph, Tabs, Row, Cell, Table};

use crate::enums::InputMode;
use crate::structs::{PassMan, Password};
use crate::widgets;


pub fn interface<B: Backend>(f: &mut Frame<B>, state: &mut PassMan) {
    //
    //  MAIN LAYOUT
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
    //  MENU
    //
    let menu_tabs = widgets::menu::new();
    f.render_widget(menu_tabs, parent_chunk[0]);

    //
    //  NEW PASSWORD
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

    let inputs = widgets::new_password::new(&state);
    
    f.render_widget(inputs.0, left_block_layout[0]); // Title
    f.render_widget(inputs.1, left_block_layout[1]); // Username
    f.render_widget(inputs.2, left_block_layout[2]); // Password
    f.render_widget(inputs.3, left_block_layout[3]); // Button

    //
    //  HELP
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
            
            let help_text = widgets::help::new();

            f.render_widget(help_text, right_block_layout[0]);
        },
        _ => {}
    }

    //
    //  SEARCH
    //
    match state.mode {
        InputMode::Search => {
            let right_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(3),
                    ]
                )
                .split(right_section[0]);

            let searchs = widgets::search::new(&state);

            f.render_widget(searchs.0, right_layout[0]); // Search Input
            f.render_widget(searchs.1, right_layout[1]); // List
        },
        _ => {}
    }

    //
    //  LIST
    //
    match state.mode {
        InputMode::List => {
            let right_layout = Layout::default()
                .constraints(
                    [
                        Constraint::Length(4),
                        Constraint::Min(3),
                    ].as_ref()
                )
                .split(right_section[0]);

            let list = widgets::list::new(&state);

            f.render_widget(list.1, right_layout[0]); // Table
            f.render_stateful_widget(list.0, right_layout[1], &mut state.list_state); // List
        },
        _ => {}
    }
}