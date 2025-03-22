use iff_rs::{IFFFile, parse_iff};
use std::fs::File;
use std::io::{self, Error, Write};

fn create_sample_iff() -> Result<(), Error> {
    let mut file = File::create("sample.iff")?;
    file.write_all(b"FORM")?;
    file.write_all(&[0, 0, 0, 8])?;
    file.write_all(b"DATA")?;
    file.write_all(&[0, 0, 0, 4])?;
    file.write_all(b"abcd")?;
    Ok(())
}

#[test]
fn test_parse_simple_iff() {
    create_sample_iff().expect("Failed to create sample IFF file");

    let file: File = File::open("sample.iff").expect("Failed to open sample IFF file");
    let iff = parse_iff(file).expect("Failed to parse IFF file");

    assert_eq!(iff.chunks.len(), 1);
    let chunk = &iff.chunks[0];
    assert_eq!(chunk.id, u32::from_be_bytes(*b"DATA"));
    assert_eq!(chunk.len, 4);
    assert_eq!(chunk.data, b"abcd");
}
