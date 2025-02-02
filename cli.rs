mod parser;
mod reporter;
mod config;
mod metrics;
mod search_history;

use std::env;
use parser::{parse_integers, parse_target};
use reporter::Reporter;
use config::{Config, create_default_config_file};
use metrics::Metrics;
use search_history::SearchHistory;
use std::time::Instant;
use std::path::Path;

pub struct Cli {
    pub args: Vec<String>,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            args: env::args().collect(),
        }
    }

    pub fn get_arg(&self, index: usize) -> Option<&String> {
        self.args.get(index)
    }
}

fn binary_search(arr: &[i32], target: i32, metrics: &mut Metrics) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() as isize - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_value = arr[mid as usize];

        metrics.increment("comparisons");

        if mid_value == target {
            return Some(mid as usize);
        } else if mid_value < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

fn main() {
    let cli = Cli::new();
    if cli.args.len() < 3 {
        println!("Usage: binary_search_cli <target> <list of integers>");
        return;
    }

    let config_path = "config.txt";
    if !Path::new(config_path).exists() {
        if let Err(e) = create_default_config_file(config_path) {
            println!("Failed to create default config file: {}", e);
            return;
        }
    }

    let config = match Config::load_from_file(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("Failed to load config: {}", e);
            return;
        }
    };

    let target = match parse_target(&cli.args[1]) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let arr = match parse_integers(&cli.args[2..]) {
        Ok(a) => a,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if arr.is_empty() {
        println!("No valid integers provided for the array");
        return;
    }

    let mut metrics = Metrics::new();
    let search_history = SearchHistory::new("search_history.txt");
    let start_time = Instant::now();
    let result = binary_search(&arr, target, &mut metrics);
    let duration = start_time.elapsed();

    match result {
        Some(index) => println!("Found target {} at index {}", target, index),
        None => println!("Target {} not found", target),
    }

    let report_content = match result {
        Some(index) => format!(
            "Target {} found at index {}. Search took {:.2?} milliseconds. Comparisons: {}",
            target, index, duration.as_millis(), metrics.get("comparisons").unwrap_or(&0)
        ),
        None => format!(
            "Target {} not found. Search took {:.2?} milliseconds. Comparisons: {}",
            target, duration.as_millis(), metrics.get("comparisons").unwrap_or(&0)
        ),
    };

    if let Err(e) = search_history.add_entry(&report_content) {
        println!("Failed to add entry to search history: {}", e);
    }

    let reporter = Reporter::new(&config.result_file);
    if let Err(e) = reporter.generate_report(&report_content) {
        println!("Failed to write report: {}", e);
    }
}