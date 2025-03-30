//! A crate that reads IFF files.

use std::fs::File;
use std::io::BufReader;

use crate::iff::process_iff;
pub use crate::{error::IFFError, iff::IFFFile};

mod chunk;
mod error;
mod iff;

/// This parses the iff file and returns a result.
pub fn parse_iff(file: File) -> Result<IFFFile, IFFError> {
    let reader = BufReader::new(file);
    process_iff(reader)
}
