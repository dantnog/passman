use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph, Tabs};

use crate::structs::PassMan;


pub fn interface<B: Backend>(f: &mut Frame<B>, state: &mut PassMan) {
    //
    //  LAYOUT
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


    //
    // WIDGETS - HOME
    //
    let menu_options: Vec<&str> = vec![
        "Insert", "Show", "Help", "Quit"
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

}