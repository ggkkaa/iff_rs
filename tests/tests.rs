use iff_rs::{IFFFile, parse_iff};
use std::fs::File;
use std::io::{self, Error, Write};

fn create_sample_iff() -> Result<(), Error> {
    let mut file = File::create("test-files/sample.iff")?;
    file.write_all(b"FORM")?;
    file.write_all(&[0, 0, 0, 8])?;
    file.write_all(b"DATA")?;
    file.write_all(&[0, 0, 0, 4])?;
    file.write_all(b"abcd")?;
    Ok(())
}

fn create_dual_sample_iff() -> Result<(), Error> {
    let mut file = File::create("test-files/dual_sample.iff")?;
    file.write_all(b"FORM")?;
    file.write_all(&[0, 0, 0, 8])?;
    file.write_all(b"DATA")?;
    file.write_all(&[0, 0, 0, 4])?;
    file.write_all(b"abcd")?;
    file.write_all(b"DATA")?;
    file.write_all(&[0, 0, 0, 4])?;
    file.write_all(b"efgh")?;
    Ok(())
}

fn create_empty_iff() -> Result<(), Error> {
    let _file = File::create("test-files/empty.iff")?;
    Ok(())
}

#[test]
fn test_parse_simple_iff() {
    create_sample_iff().expect("Failed to create sample IFF file");

    let file: File = File::open("test-files/sample.iff").expect("Failed to open sample IFF file");
    let iff = parse_iff(file).expect("Failed to parse IFF file");

    assert_eq!(iff.chunks.len(), 1);
    let chunk = &iff.chunks[0];
    assert_eq!(chunk.id, u32::from_be_bytes(*b"DATA"));
    assert_eq!(chunk.len, 4);
    assert_eq!(chunk.data, b"abcd");
    
}

#[test]
fn test_parse_empty_iff() {
    create_empty_iff().expect("Failed to create empty IFF file");

    let file: File = File::open("test-files/empty.iff").expect("Failed to open empty IFF file");
    let iff = parse_iff(file);

    assert!(iff.is_err(), "Empty IFF file should return an error");
}

#[test]
fn test_parse_dual_iff() {
    create_dual_sample_iff().expect("Failed to create sample IFF file");

    let file: File = File::open("test-files/dual_sample.iff").expect("Failed to open sample IFF file");
    let iff = parse_iff(file).expect("Failed to parse IFF file");

    assert_eq!(iff.chunks.len(), 2);
    let mut chunk = &iff.chunks[0];
    assert_eq!(chunk.id, u32::from_be_bytes(*b"DATA"));
    assert_eq!(chunk.len, 4);
    assert_eq!(chunk.data, b"abcd");
    
    chunk = &iff.chunks[1];
    assert_eq!(chunk.id, u32::from_be_bytes(*b"DATA"));
    assert_eq!(chunk.len, 4);
    assert_eq!(chunk.data, b"efgh");
}

#[test]
fn test_no_errors_parse_ilbm() {

    let file: File = File::open("examples/harrison.iff").expect("Failed to open sample IFF file");
    let iff = parse_iff(file).expect("Failed to parse IFF file");

    assert_eq!(iff.chunks.len(), 6);
    let mut chunk = &iff.chunks[0];
    assert_eq!(chunk.id, u32::from_be_bytes(*b"DATA"));
    assert_eq!(chunk.len, 4);
    assert_eq!(chunk.data, b"abcd");
    
    chunk = &iff.chunks[1];
    assert_eq!(chunk.id, u32::from_be_bytes(*b"DATA"));
    assert_eq!(chunk.len, 4);
    assert_eq!(chunk.data, b"efgh");
}