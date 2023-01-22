use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Tabs};

pub fn new(index: usize) -> Tabs<'static> {
    let tabs_options: Vec<&str> = vec![
        "Home", "Insert", "Search", "List", "Help", "Quit"
    ];

    let tabs_spans = tabs_options
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .add_modifier(Modifier::UNDERLINED)
                ),
                Span::styled(
                    rest,
                    Style::default()
                )
            ])
        })
        .collect();

    let tabs = Tabs::new(tabs_spans)
        .block(
            Block::default()
                .title("Tabs")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .select(index)
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        )
        .divider(Span::raw("|"));

    tabs
}