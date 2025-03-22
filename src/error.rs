use std::fmt;
use std::io;

#[derive(Debug)]
pub enum IFFError {
    EmptyFile,
    Io(io::Error),
    InvalidFormat(String),
}

impl fmt::Display for IFFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IFFError::EmptyFile => write!(f, "Empty file"),
            IFFError::Io(e) => write!(f, "IO error: {}", e),
            IFFError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
        }
    }
}

impl std::error::Error for IFFError {}

impl From<io::Error> for IFFError {
    fn from(e: io::Error) -> Self {
        IFFError::Io(e)
    }
}