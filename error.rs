use std::fmt;

#[derive(Debug)]
pub enum SearchError {
    InvalidInput(String),
    IoError(std::io::Error),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            SearchError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<std::io::Error> for SearchError {
    fn from(err: std::io::Error) -> Self {
        SearchError::IoError(err)
    }
}