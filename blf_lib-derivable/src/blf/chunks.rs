mod blf_chunk;

pub use blf_chunk::*;

pub trait Serializable {
    fn encode(&self, buffer: &[u8]);
    fn decode(buffer: &[u8]) -> Self;
}