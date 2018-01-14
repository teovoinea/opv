use image::{
    ImageBuffer,
    Rgba
};

use transcode::Chunk;

pub fn frame_to_chunks(frame: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<Chunk> {
    let raw_bytes = frame.into_raw();
    let mut chunks = Vec::new();
    for (i, chunk) in raw_bytes.chunks(1500).enumerate() {
        let t_chunk = Chunk {
            index: i,
            data: chunk.to_vec()
        };
        chunks.push(t_chunk);
    }
    chunks
}