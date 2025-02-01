pub fn validate_integer(input: &str) -> Result<i32, String> {
    input.trim().parse::<i32>().map_err(|_| "Invalid integer".to_string())
}

pub fn validate_array(input: &str) -> Result<Vec<i32>, String> {
    input.split(',')
        .map(|s| s.trim().parse::<i32>().map_err(|_| "Invalid integer in array".to_string()))
        .collect()
}