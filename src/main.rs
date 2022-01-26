use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?} \r", key)
                    } else {
                        println!("{:?} ({})\r", key, c)
                    }
                }
                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", key),
            },
            Err(err) => die(err),
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{e}");
}
