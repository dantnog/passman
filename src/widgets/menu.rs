use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Tabs};

pub fn new() -> Tabs<'static> {
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

    menu_tabs
}