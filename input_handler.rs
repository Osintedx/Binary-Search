use std::io;

/// Reads a line of input from the user.
pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

/// Parses a comma-separated string into a vector of integers.
pub fn parse_array(input: &str) -> Result<Vec<i32>, &'static str> {
    input.split(',')
        .map(|s| s.trim().parse().map_err(|_| "Invalid integer"))
        .collect()
}

/// Parses a yes/no input into a boolean.
pub fn parse_bool(input: &str) -> Result<bool, &'static str> {
    match input.to_lowercase().as_str() {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err("Please enter 'yes' or 'no'"),
    }
}