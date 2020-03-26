use std::env;
use std::io;
use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{List, Text, Widget};
use tui::*;

mod event;
mod qiita;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("QIITA_TOKEN").unwrap();

    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = termion::screen::AlternateScreen::from(stdout);
    let backend = backend::TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor();

    let client = qiita::QiitaClient::new(&format!("Bearer {}", api_key));
    // let trends = client.trends().unwrap();
    let items = client.auth_items().unwrap();

    let events = event::Events::new();

    loop {
        terminal.draw(|mut f| {
            let mut items = List::new(
                // trends
                //     .iter()
                //     .map(|trend| Text::raw(format!("{}", &trend.node.title))),
                items
                    .iter()
                    .map(|item| Text::raw(format!("{}", &item.title))),
            );
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
                .split(f.size());
            f.render(&mut items, chunks[0])
        })?;

        match events.next()? {
            event::Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                _ => {}
            },
            _ => {}
        };
    }

    Ok(())
}
