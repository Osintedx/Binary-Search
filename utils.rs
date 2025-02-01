use std::fs::OpenOptions;
use std::io::{self, Write};

/// Logs user actions to a file.
pub fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("search_log.txt")
        .expect("Unable to open log file");
    writeln!(file, "{}", action).expect("Unable to write to log file");
}

/// Saves search results to a file.
pub fn save_result(arr: &[i32], target: i32, result: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("search_results.txt")
        .expect("Unable to open results file");
    writeln!(file, "Array: {:?}, Target: {}, Result: {}", arr, target, result)
        .expect("Unable to write to results file");
}

/// Checks if the array is sorted in the specified order.
pub fn is_sorted(arr: &[i32], ascending: bool) -> bool {
    if ascending {
        arr.windows(2).all(|w| w[0] <= w[1])
    } else {
        arr.windows(2).all(|w| w[0] >= w[1])
    }
}