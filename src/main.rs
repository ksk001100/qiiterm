use std::io;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::widgets::Widget;
use tui::*;

const MESSAGE: &str = "Merry Christmas !!";

struct Label<'a> {
    x: u16,
    y: u16,
    text: &'a str,
    style: style::Style,
}

impl<'a> Default for Label<'a> {
    fn default() -> Label<'a> {
        Label {
            x: 0,
            y: 0,
            text: "",
            style: style::Style::default(),
        }
    }
}

impl<'a> Label<'a> {
    fn text(&mut self, text: &'a str) -> &mut Label<'a> {
        self.text = text;
        self
    }
    fn position(&mut self, x: u16, y: u16) -> &mut Label<'a> {
        self.x = x;
        self.y = y;
        self
    }
    fn style(&mut self, style: style::Style) -> &mut Label<'a> {
        self.style = style;
        self
    }
}

impl<'a> Widget for Label<'a> {
    fn draw(&mut self, area: layout::Rect, buf: &mut buffer::Buffer) {
        buf.set_string(
            area.left() + self.x,
            area.top() + self.y,
            self.text,
            self.style,
        );
    }
}

fn main() {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = termion::screen::AlternateScreen::from(stdout);
    let backend = backend::TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor();
    terminal
        .draw(|mut f| {
            let size = f.size();
            Label::default().text("Hello World !!").render(&mut f, size);
        })
        .unwrap();

    let stdin = io::stdin();
    for c in stdin.keys() {
        match c {
            Ok(termion::event::Key::Char('m')) => {
                terminal.clear();
                terminal
                    .draw(|mut f| {
                        let size = f.size();
                        let x = size.width / 2 - (MESSAGE.len() / 2) as u16;
                        let y = size.height / 2;
                        let style = style::Style::default().fg(style::Color::Blue);
                        Label::default()
                            .text(MESSAGE)
                            .position(x, y)
                            .style(style)
                            .render(&mut f, size);
                    })
                    .unwrap();
            }
            Ok(termion::event::Key::Ctrl('c')) => break,
            _ => {}
        }
    }
}
