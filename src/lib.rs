//! A crate that reads IFF files.

use std::io::BufReader;
use std::fs::File;

pub use crate::{iff::IFFFile, error::IFFError};
use crate::iff::process_iff;

mod chunk;
mod iff;
mod error;

/// This parses the iff file and returns a result.
pub fn parse_iff(file: File) -> Result<IFFFile, IFFError> {
    let reader = BufReader::new(file);
    process_iff(reader)
}
