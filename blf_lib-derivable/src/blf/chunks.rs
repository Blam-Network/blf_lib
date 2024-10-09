use bincode::Encode;
use crate::blf::BlfFile;
use crate::types::chunk_signature::chunk_signature;
use crate::types::chunk_version::chunk_version;

pub trait Serializable: BlfChunk {
    fn encode(&self) -> Vec<u8>;
    fn decode(buffer: &[u8]) -> Self;

    fn write(&mut self) -> Vec<u8> {
        let mut encoded_chunk = self.encode();
        let header = crate::blf::s_blf_header::s_blf_header {
            signature: Self::get_signature(),
            major_version: Self::get_version().major,
            minor_version: Self::get_version().minor,
            chunk_size: encoded_chunk.len() as u32,
        };

        let bincode_config = bincode::config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();

        let mut encoded_header = bincode::encode_to_vec(header, bincode_config).unwrap();
        encoded_header.append(&mut encoded_chunk);

        encoded_header
    }
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
    fn decode(&self, signature: &chunk_signature, version: chunk_version, buffer: &[u8]) -> Result<Box<dyn DynamicBlfChunk>, &'static str>;
}