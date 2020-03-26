use std::env;
use std::io;
use termion::{
    self,
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, SelectableList, Text, Widget},
    Terminal,
};

mod qiita;
mod util;

use util::event;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("QIITA_TOKEN").unwrap();

    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor()?;

    let client = qiita::QiitaClient::new(&format!("Bearer {}", api_key));
    // let articles = client.trends().unwrap();
    let articles = client.auth_items().unwrap();

    let events = event::Events::new();

    let mut selected_item_index = 0;
    let mut show_item_index = 0;

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let block = Block::default().borders(Borders::ALL);
            SelectableList::default()
                .block(block)
                .items(
                    &articles
                        .iter()
                        .map(|item| &item.title)
                        .collect::<Vec<&String>>(),
                )
                .select(Some(selected_item_index))
                .highlight_symbol(">>")
                .render(&mut f, chunks[0]);

            let block = Block::default().borders(Borders::ALL);
            List::new(
                (&articles[show_item_index].body)
                    .split("\n")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|b| Text::raw(*b)),
            )
            .block(block)
            .render(&mut f, chunks[1]);
        })?;

        match events.next()? {
            event::Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                Key::Char('k') | Key::Up => {
                    if selected_item_index > 0 {
                        selected_item_index -= 1;
                    }
                }
                Key::Char('j') | Key::Down => {
                    if selected_item_index + 1 < articles.len() {
                        selected_item_index += 1;
                    }
                }
                Key::Char('\n') => {
                    show_item_index = selected_item_index;
                }
                _ => {}
            },
            _ => {}
        };
    }

    Ok(())
}
