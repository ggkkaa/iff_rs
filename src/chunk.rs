type ChunkId = u32;

/// IFFChunk is a chunk of data in an IFF file.
#[derive(Debug)]
pub struct IFFChunk {
    pub id: ChunkId,
    pub len: u32,
    pub data: Vec<u8>,
}

impl IFFChunk {
    pub fn new(id: ChunkId, len: u32, data: Vec<u8>) -> Self {
        IFFChunk { id, len, data }
    }
}
