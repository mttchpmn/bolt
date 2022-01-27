use crate::Terminal;
use termion::event::Key;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialise terminal"),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                die(&err);
            }

            if self.should_quit {
                break;
            }

            if let Err(err) = self.process_keypress() {
                die(&err);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::set_cursor_position(0, 0);

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::set_cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            Terminal::clear_current_line();
            println!("~\r");
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;

        if let Key::Ctrl('q') = pressed_key {
            self.should_quit = true;
        }
        Ok(())
    }
}

fn die(e: &std::io::Error) {
    println!("{}", termion::clear::All);
    panic!("{e}");
}
