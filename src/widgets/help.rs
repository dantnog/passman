use tui::widgets::Paragraph;

pub fn new<'a>() -> Paragraph<'a> {
    const HELP_TEXT: &str = 
r#" H              Help
 I              Insert new
 L              List
 S              Search
 Q              Quit (Normal Mode)
 Del            Delete
 Esc            Return to Normal Mode
 Tab            Next field
 Shift+Tab      Prev field
 Enter          Active button
 "#;

    let help_text = Paragraph::new(HELP_TEXT);

    help_text
}