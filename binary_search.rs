mod config;
mod error;
mod input_handler;
mod logger;
mod result_saver;
mod search;
mod search_history;
mod utils;

use config::Config;
use error::SearchError;
use input_handler::{parse_array, parse_bool, read_input};
use logger::Logger;
use result_saver::ResultSaver;
use search::binary_search;
use search_history::SearchHistory;
use utils::is_sorted;

fn main() -> Result<(), SearchError> {
    let config = Config::load_from_file("config.txt").unwrap_or(Config::new("search_log.txt", "search_results.txt"));
    let history = SearchHistory::new("search_history.txt");
    let logger = Logger::new(&config.log_file);
    let result_saver = ResultSaver::new(&config.result_file);

    loop {
        // Read and parse the array
        let input = read_input("Enter a sorted array of integers (comma separated):");
        let arr = parse_array(&input).map_err(SearchError::InvalidInput)?;

        // Read and parse the sorting order
        let input = read_input("Is the array sorted in ascending order? (yes/no):");
        let ascending = parse_bool(&input).map_err(SearchError::InvalidInput)?;

        // Validate if the array is sorted in the specified order
        if !is_sorted(&arr, ascending) {
            println!("The array is not sorted in the specified order.");
            continue;
        }

        // Read and parse the target value
        let input = read_input("Enter the target value:");
        let target: i32 = input.trim().parse().map_err(|_| SearchError::InvalidInput("Invalid integer".to_string()))?;

        // Perform the binary search and display the result
        let result = match binary_search(&arr, target, ascending) {
            Some(index) => {
                let msg = format!("Target found at index: {}", index);
                println!("{}", msg);
                msg
            },
            None => {
                let msg = "Target not found in the array".to_string();
                println!("{}", msg);
                msg
            }
        };

        // Log the search result
        logger.log(&format!("Array: {:?}, Target: {}, Result: {}", arr, target, result))?;

        // Save the result to a file
        result_saver.save(&arr, target, &result)?;

        // Add entry to search history
        history.add_entry(&format!("Array: {:?}, Target: {}, Result: {}", arr, target, result))?;

        // Ask the user if they want to perform another search
        let input = read_input("Do you want to perform another search? (yes/no):");
        if input.to_lowercase() != "yes" {
            break;
        }
    }

    Ok(())
}