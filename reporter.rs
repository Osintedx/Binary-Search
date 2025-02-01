use std::fs::File;
use std::io::{self, Write};

pub struct Reporter {
    file_path: String,
}

impl Reporter {
    pub fn new(file_path: &str) -> Self {
        Reporter {
            file_path: file_path.to_string(),
        }
    }

    pub fn generate_report(&self, content: &str) -> io::Result<()> {
        let mut file = File::create(&self.file_path)?;
        writeln!(file, "{}", content)?;
        Ok(())
    }
}