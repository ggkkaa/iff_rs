use crate::chunk::IFFChunk;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{self, Read};

pub struct IFFFile {
    pub chunks: Vec<IFFChunk>,
}

impl IFFFile {
    pub fn new() -> IFFFile {
        IFFFile { chunks: Vec::new() }
    }

    pub fn add_chunk(&mut self, chunk: IFFChunk) {
        self.chunks.push(chunk);
    }
}

pub fn process_iff<R: Read>(reader: R) -> Result<IFFFile, io::Error> {
    let mut iff_file = IFFFile::new();
    let mut reader = io::BufReader::new(reader);

    let mut header = [0u8; 4];
    reader.read_exact(&mut header)?;
    let _ = reader.read_u32::<BigEndian>()?;

    while let Ok(chunk) = parse_chunk(&mut reader) {
        iff_file.add_chunk(chunk);
    }

    Ok(iff_file)
}

fn parse_chunk<R: Read>(reader: &mut R) -> Result<IFFChunk, io::Error> {
    let mut id_bytes = [0u8; 4];
    reader.read_exact(&mut id_bytes)?;

    let id = u32::from_be_bytes(id_bytes);

    let len = reader.read_u32::<BigEndian>()?;
    let mut data = vec![0; len as usize];
    reader.read_exact(&mut data)?;

    Ok(IFFChunk::new(id, len, data))
}
