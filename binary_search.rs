use std::fs::OpenOptions;
use std::io::{self, Write};

/// Performs a binary search on a sorted array.
fn binary_search(arr: &[i32], target: i32, ascending: bool) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() as i32 - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_val = arr[mid as usize];

        if mid_val == target {
            return Some(mid as usize);
        } else if (mid_val < target && ascending) || (mid_val > target && !ascending) {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

/// Checks if the array is sorted in the specified order.
fn is_sorted(arr: &[i32], ascending: bool) -> bool {
    if ascending {
        arr.windows(2).all(|w| w[0] <= w[1])
    } else {
        arr.windows(2).all(|w| w[0] >= w[1])
    }
}

/// Logs user actions to a file.
fn log_action(action: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("search_log.txt")
        .expect("Unable to open log file");
    writeln!(file, "{}", action).expect("Unable to write to log file");
}

fn main() {
    loop {
        let mut input = String::new();

        // Prompt the user to enter a sorted array of integers
        println!("Enter a sorted array of integers (comma separated):");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let arr: Vec<i32> = match input.trim().split(',')
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<_>, _>>() {
            Ok(arr) => arr,
            Err(_) => {
                println!("Please enter valid integers.");
                continue;
            }
        };

        input.clear();

        // Ask the user if the array is sorted in ascending order
        println!("Is the array sorted in ascending order? (yes/no):");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let ascending = match input.trim().to_lowercase().as_str() {
            "yes" => true,
            "no" => false,
            _ => {
                println!("Please enter 'yes' or 'no'.");
                continue;
            }
        };

        // Validate if the array is sorted in the specified order
        if !is_sorted(&arr, ascending) {
            println!("The array is not sorted in the specified order.");
            continue;
        }

        input.clear();

        // Prompt the user to enter the target value
        println!("Enter the target value:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let target: i32 = match input.trim().parse() {
            Ok(target) => target,
            Err(_) => {
                println!("Please enter a valid integer.");
                continue;
            }
        };

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
        log_action(&format!("Array: {:?}, Target: {}, Result: {}", arr, target, result));

        // Save the result to a file
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("search_results.txt")
            .expect("Unable to open results file");
        writeln!(file, "Array: {:?}, Target: {}, Result: {}", arr, target, result)
            .expect("Unable to write to results file");

        input.clear();

        // Ask the user if they want to perform another search
        println!("Do you want to perform another search? (yes/no):");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() != "yes" {
            break;
        }
    }
}