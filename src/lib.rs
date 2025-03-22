//! A crate that reads IFF files.

use std::io::BufReader;
use std::{fs::File, io::Error};

pub use crate::iff::IFFFile;
use crate::iff::process_iff;

mod chunk;
mod iff;
mod error;

/// This parses the iff file and returns a result.
pub fn parse_iff(file: File) -> Result<IFFFile, Error> {
    let reader = BufReader::new(file);
    process_iff(reader)
}
