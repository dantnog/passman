use tui::widgets::Paragraph;

pub fn new<'a>() -> Paragraph<'a> {
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

    help_text
}