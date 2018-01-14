use image::{
    ImageBuffer,
    Rgba
};

use transcode::Chunk;

lazy_static! {
    static ref frame_buffer: Vec<u8> = Vec::new();
}

pub fn chunks_to_frame(chunks: Vec<Chunk>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut t_buf: Vec<u8> = Vec::new();
    for mut chunk in chunks {
        t_buf.append(&mut chunk.data);
    }
    let frame: ImageBuffer<Rgba<u8>, Vec<u8>>;
    frame = ImageBuffer::from_raw(640, 480, t_buf).unwrap();
    frame
}