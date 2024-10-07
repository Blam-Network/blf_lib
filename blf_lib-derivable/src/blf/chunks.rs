use crate::types::chunk_signature::chunk_signature;

pub trait Serializable {
    fn encode(&self) -> &[u8];
    fn decode(buffer: &[u8]) -> Self;
}

pub trait BlfChunk {
    fn get_signature() -> chunk_signature;
    fn get_version() -> [u16; 2];
}

pub trait DynamicBlfChunk {}

pub trait TitleAndBuild {
    fn get_title() -> &'static str;

    fn get_build_string() -> &'static str;
}

pub trait ChunkFactory {
    fn decode_chunk(&self, signature: &chunk_signature, major_version: u16, minor_version: u16, buffer: &[u8]) -> Result<Box<dyn DynamicBlfChunk>, &'static str>;
}