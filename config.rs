use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub struct Config {
    pub log_file: String,
    pub result_file: String,
}

impl Config {
    pub fn new(log_file: &str, result_file: &str) -> Self {
        Config {
            log_file: log_file.to_string(),
            result_file: result_file.to_string(),
        }
    }

    pub fn load_from_file(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lines: Vec<&str> = contents.lines().collect();
        Ok(Config {
            log_file: lines.get(0).unwrap_or(&"search_log.txt").to_string(),
            result_file: lines.get(1).unwrap_or(&"search_results.txt").to_string(),
        })
    }

    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "{}\n{}", self.log_file, self.result_file)?;
        Ok(())
    }
}