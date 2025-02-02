use std::fs::File;
use std::io::{self, Read, Write};

pub struct Config {
    pub result_file: String,
}

impl Config {
    pub fn load_from_file(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lines: Vec<&str> = contents.lines().collect();
        Ok(Config {
            result_file: lines.get(1).unwrap_or(&"search_results.txt").to_string(),
        })
    }
}

pub fn create_default_config_file(path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "search_log.txt\nsearch_results.txt")?;
    Ok(())
}