//use bincode::{serialize, deserialize, Infinite};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Chunk {
    index: usize,
    data: Vec<u8>
}

pub mod encode;
pub mod decode;