#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod document;
mod row;
mod logger;
mod config;

use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;
pub use document::Document;
pub use row::Row;
pub use logger::Logger;
use crate::config::Config;

fn main() {
    // TODO - Use default config if load fails
    let config = Config::load("config.json").expect("Failed to load config");
    let logger = Logger::new("log.txt");

    Editor::new(config, &logger).run();
}

