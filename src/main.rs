#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod document;
mod row;
mod logger;

use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;
pub use document::Document;
pub use row::Row;
pub use logger::Logger;

fn main() {
    let logger = Logger::new("log.txt");
    Editor::new(&logger).run();
}

