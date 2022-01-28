use std::fs;

pub struct Logger {
    log_filepath: String
}

impl Logger {
    pub fn new(filepath: String) -> Self {
        Self {
            log_filepath: filepath
        }
    }

    pub fn info(&self, msg: &str) {
        self.log("INFO", msg);
    }
    pub fn warn(&self, msg: &str) {}
    pub fn error(&self, msg: &str) {}
    pub fn debug(&self, msg: &str) {}

    fn log(&self, log_level: &str, msg: &str) {
        let formatted_msg = &self.format_log_message(log_level, msg);
        self.write_to_log_file(formatted_msg);
    }

    fn format_log_message(&self, log_level: &str, msg: &str) -> String {
        format!("[{}]: {}\n", log_level, msg)
    }

    fn write_to_log_file(&self, msg: &str) {
        fs::write(&self.log_filepath, msg).expect("Failed to write to log file");
    }
}
