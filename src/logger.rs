use std::fs;

use chrono::prelude::*;

pub struct Logger {
    log_filepath: String
}

impl Logger {
    pub fn new(filepath: &str) -> Self {
        Self {
            log_filepath: String::from(filepath)
        }
    }

    pub fn debug(&self, msg: &str) {self.log("DEBUG", msg);}
    pub fn info(&self, msg: &str) {
        self.log("INFO", msg);
    }
    pub fn warn(&self, msg: &str) {self.log("WARN", msg);}
    pub fn error(&self, msg: &str) { self.log("ERROR", msg);}

    fn log(&self, log_level: &str, msg: &str) {
        let formatted_msg = &self.format_log_message(log_level, msg);
        self.write_to_log_file(formatted_msg);
    }

    fn format_log_message(&self, log_level: &str, msg: &str) -> String {
        let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        format!("[{}]: [{}]: {}\n", time, log_level, msg)
    }

    fn write_to_log_file(&self, msg: &str) {
        let mut existing_content = fs::read_to_string(&self.log_filepath).expect("Failed to read log file");
        existing_content.push_str(msg);

        fs::write(&self.log_filepath, existing_content).expect("Failed to write to log file");
    }
}
