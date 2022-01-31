use crate::Document;
use crate::Logger;
use crate::Row;
use crate::Terminal;

use std::env;
use std::time::{Duration, Instant};
use termion::color;
use termion::event::Key;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(50, 50, 50);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const LINE_NUMBER_FG_COLOR: color::Rgb = color::Rgb(120, 120, 120);
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}

pub struct Editor<'a> {
    should_quit: bool,
    confirm_quit: bool,
    terminal: Terminal<'a>,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    logger: &'a Logger,
}

impl<'a> Editor<'a> {
    pub fn new(logger: &'a Logger) -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-S = Save | Ctrl-Q = quit");
        let document = if args.len() > 1 {
            let filename = &args[1];
            let doc = Document::open(&filename);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("ERR: Could not open file: {}", filename);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            confirm_quit: false,
            terminal: Terminal::new(logger).expect("Failed to initialise terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
            status_message: StatusMessage::from(initial_status),
            logger,
        }
    }

    pub fn run(&mut self) {
        self.logger.info("Editor running");
        loop {
            if let Err(err) = self.refresh_screen() {
                self.logger
                    .error(&format!("Error refreshing screen: {}", err));
                die(&err);
            }

            if self.should_quit {
                self.logger.info("Quitting");
                break;
            }

            if let Err(err) = self.process_keypress() {
                self.logger
                    .error(&format!("Error handling keypress: {}", err));
                die(&err);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::set_cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::set_cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for row_index in 0..height - 1 {
            Terminal::clear_current_line();

            if let Some(row) = self.document.row(row_index as usize + self.offset.y) {
                self.render_row(row, row_index + 1);
            } else if self.document.is_empty() && row_index == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    pub fn render_row(&self, row: &Row, index: u16) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);

        let line_number = if index < 10 {
            format!("0{}", index)
        } else {
            format!("{}", index)
        };

        // Uncomment once movement is handled
        // Terminal::set_fg_color(LINE_NUMBER_FG_COLOR);
        // print!("{}|", line_number);
        // Terminal::reset_fg_color();

        println!("{}\r", row);
    }

    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            "(modified)"
        } else {
            ""
        };
        let mut filename = String::from("[No Name]");

        if let Some(name) = &self.document.filename {
            filename = name.clone();
            filename.truncate(20);
        }
        status = format!(
            "{} - {} lines {}",
            filename,
            self.document.len(),
            modified_indicator
        );
        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Bolt editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);

        println!("{}\r", welcome_message);
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;

        match pressed_key {
            Key::Ctrl('q') => {
                if self.document.is_dirty() && !self.confirm_quit {
                    self.set_status_message(
                        "Warning: File has unsaved changes. Press Ctrl + Q again to quit.",
                    );
                    self.confirm_quit = true;

                    return Ok(());
                }
                self.should_quit = true
            }
            Key::Ctrl('s') => self.handle_save(),
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageDown
            | Key::PageUp
            | Key::Home
            | Key::End => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();

        Ok(())
    }

    fn set_status_message(&mut self, msg: &str) {
        self.status_message = StatusMessage::from(String::from(msg));
    }

    fn handle_quit(&mut self) {}

    fn handle_save(&mut self) {
        match self.document.filename {
            None => {
                let new_name = self.prompt("Save as:").unwrap_or(None);
                if new_name.is_none() {
                    self.set_status_message("Save aborted");
                    return;
                }
            }
            _ => {}
        }

        match self.document.save() {
            Ok(_) => self.set_status_message("File saved successfully"),
            Err(_) => self.set_status_message("Error saving file!"),
        }
    }

    fn prompt(&mut self, prompt: &str) -> Result<Option<String>, std::io::Error> {
        let mut result = String::new();
        loop {
            self.set_status_message(&format!("{} {}", prompt, result));
            self.refresh_screen()?;

            match Terminal::read_key()? {
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len() - 1);
                    }
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
        }
        self.set_status_message("");
        if result.is_empty() {
            return Ok(None);
        }

        Ok(Some(result))
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let terminal_height = self.terminal.size().height as usize;

        let document_height = self.document.len();
        let mut row_width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < document_height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            Key::Right => {
                if x < row_width {
                    x += 1;
                } else if y < document_height {
                    y += 1;
                    x = 0;
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < document_height {
                    y + terminal_height as usize
                } else {
                    0
                }
            }
            Key::PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                }
            }
            Key::Home => x = 0,
            Key::End => x = row_width,

            _ => (),
        }
        row_width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > row_width {
            x = row_width;
        }
        self.cursor_position = Position { x, y }
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
}

fn die(e: &std::io::Error) {
    println!("{}", termion::clear::All);
    panic!("{e}");
}
