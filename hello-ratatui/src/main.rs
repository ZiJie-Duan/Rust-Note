use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut counter: i32 = 0;

    loop {
        counter += 1;
        terminal.draw(|frame| {
            let greeting = Paragraph::new(
                "Hello Ratatui! (press 'Q' to quit)".to_string() + &counter.to_string(),
            )
            .white()
            .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('Q') {
                return Ok(());
            }
        }
    }
}
