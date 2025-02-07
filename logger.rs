use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct Logger {
    file_path: String,
}

impl Logger {
    pub fn new(file_path: &str) -> Self {
        Logger {
            file_path: file_path.to_string(),
        }
    }

    pub fn log(&self, message: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path)?;
        writeln!(file, "{}", message)?;
        Ok(())
    }
}