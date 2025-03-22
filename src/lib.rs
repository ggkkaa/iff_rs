use std::{fmt::Error, fs::File};
use std::io::BufReader;

pub use crate::iff::{process_iff, IFFFile};

mod iff;
mod chunk;

pub fn parse_iff(file: File) -> Result<IFFFile, Error> {
        let reader = BufReader::new(file);

        //process_iff(reader);

        Ok(IFFFile::new())
}