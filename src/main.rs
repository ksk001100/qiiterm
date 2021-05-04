use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    env, fs, io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
        Wrap,
    },
    Terminal,
};

mod qiita;

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(PartialEq, Eq)]
enum Tab {
    Left,
    Right,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let api_key = env::var("QIITA_TOKEN").unwrap();

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let client = qiita::QiitaClient::new(&format!("Bearer {}", api_key));
    // let articles = client.trends().unwrap();
    let articles = client.auth_items().unwrap();

    let mut selected_item_index = 0;
    let mut show_item_index = 0;
    let mut scroll: u16 = 0;
    let mut tab_state = Tab::Left;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let color = match tab_state {
                Tab::Left => Color::Red,
                Tab::Right => Color::White,
            };
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color))
                .title("My articles");
            let items: Vec<ListItem> = articles
                .iter()
                .map(|item| ListItem::new(item.title.to_owned()))
                .collect::<Vec<ListItem>>();
            let mut state = ListState::default();
            state.select(Some(selected_item_index));
            let list = List::new(items).block(block).highlight_symbol(">>");

            f.render_stateful_widget(list, chunks[0], &mut state);

            let color = match tab_state {
                Tab::Left => Color::White,
                Tab::Right => Color::Red,
            };
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color))
                .title("Article detail");
            let text = ansi4tui::bytes_to_text(
                termimad::text(&articles[show_item_index].body)
                    .to_string()
                    .into_bytes(),
            );
            let p = Paragraph::new(text)
                .block(block)
                .wrap(Wrap { trim: true })
                .scroll((scroll, 0));
            f.render_widget(p, chunks[1]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    break;
                }
                KeyCode::Char('j') => match tab_state {
                    Tab::Left if selected_item_index + 1 < articles.len() => {
                        selected_item_index += 1
                    }
                    Tab::Right => scroll += 1,
                    _ => {}
                },
                KeyCode::Char('k') => match tab_state {
                    Tab::Left if selected_item_index > 0 => selected_item_index -= 1,
                    Tab::Right if scroll > 0 => scroll -= 1,
                    _ => {}
                },
                KeyCode::Tab => match tab_state {
                    Tab::Left => tab_state = Tab::Right,
                    Tab::Right => tab_state = Tab::Left,
                },
                KeyCode::Enter => {
                    if tab_state == Tab::Left {
                        show_item_index = selected_item_index;
                        scroll = 0;
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
