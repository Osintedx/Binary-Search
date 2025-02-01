pub fn parse_csv(input: &str) -> Result<Vec<i32>, String> {
    input.split(',')
        .map(|s| s.trim().parse::<i32>().map_err(|_| "Invalid integer in CSV".to_string()))
        .collect()
}

pub fn parse_json(input: &str) -> Result<Vec<i32>, String> {
    serde_json::from_str(input).map_err(|_| "Invalid JSON format".to_string())
}