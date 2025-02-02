use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct SearchHistory {
    file_path: String,
}

impl SearchHistory {
    pub fn new(file_path: &str) -> Self {
        SearchHistory {
            file_path: file_path.to_string(),
        }
    }

    pub fn add_entry(&self, entry: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path)?;
        writeln!(file, "{}", entry)
    }
}