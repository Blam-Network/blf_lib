use crate::types::chunk_signature::chunk_signature;
use crate::types::chunk_version::chunk_version;

pub trait Serializable {
    fn encode_chunk(&self) -> Vec<u8>;
    fn decode_chunk(buffer: &[u8]) -> Self;
}

pub trait BlfChunk {
    fn get_signature() -> chunk_signature;
    fn get_version() -> chunk_version;
}

pub trait DynamicBlfChunk {}

pub trait TitleAndBuild {
    fn get_title() -> &'static str;

    fn get_build_string() -> &'static str;
}

pub trait ChunkFactory {
    fn decode_chunk(&self, signature: &chunk_signature, version: chunk_version, buffer: &[u8]) -> Result<Box<dyn DynamicBlfChunk>, &'static str>;
}