use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct ResultSaver {
    file_path: String,
}

impl ResultSaver {
    pub fn new(file_path: &str) -> Self {
        ResultSaver {
            file_path: file_path.to_string(),
        }
    }

    pub fn save(&self, arr: &[i32], target: i32, result: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path)?;
        writeln!(file, "Array: {:?}, Target: {}, Result: {}", arr, target, result)?;
        Ok(())
    }
}