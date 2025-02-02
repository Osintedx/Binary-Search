pub fn parse_integers(args: &[String]) -> Result<Vec<i32>, String> {
    let mut integers = Vec::new();
    for arg in args {
        match arg.parse::<i32>() {
            Ok(num) => integers.push(num),
            Err(_) => return Err(format!("Invalid integer: {}", arg)),
        }
    }
    Ok(integers)
}

pub fn parse_target(arg: &str) -> Result<i32, String> {
    match arg.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Invalid target: {}", arg)),
    }
}