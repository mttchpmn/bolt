use crate::{Logger, Position};

use std::io::{self, stdout, Write};
use termion::{
    color,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal<'a> {
    size: Size,
    logger: &'a Logger,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl<'a> Terminal<'a> {
    pub fn new(logger: &'a Logger) -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(1), // Leave room for status bars
            },
            logger,
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn set_cursor_position(position: &Position) {
        let Position { mut x, mut y } = &position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x as u16, y as u16));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color))
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset))
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color))
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset))
    }
}
