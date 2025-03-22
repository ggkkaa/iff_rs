use std::fmt;
use std::io;

#[derive(Debug)]
pub enum IffError {
    EmptyFile,
    Io(io::error),
    InvalidFormat(String),
}

impl fmt::Display for IffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IffError::EmptyFile => write!(f, "Empty file"),
            IffError::Io(e) => write!(f, "IO error: {}", e),
            IffError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
        }
    }
}

impl std::error::Error for IffError {}

impl From<io::Error> for IffError {
    fn from(e: io::Error) -> Self {
        IffError::Io(e)
    }
}