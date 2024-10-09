use crate::types::chunk_signature::chunk_signature;
use crate::types::chunk_version::chunk_version;

pub trait BlfChunk {
    fn get_signature() -> chunk_signature;
    fn get_version() -> chunk_version;
}

pub trait DynamicBlfChunk {
    fn signature(&self) -> chunk_signature;
    fn version(&self) -> chunk_version;
}

pub trait SerializableBlfChunk: DynamicBlfChunk {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8>;
    fn decode_body(&mut self, buffer: &[u8]);

    fn write(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
        let mut encoded_chunk = self.encode_body(previously_written);
        let header = crate::blf::s_blf_header::s_blf_header {
            signature: self.signature(),
            version: self.version(),
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

pub trait ReadableBlfChunk: BlfChunk + Sized + SerializableBlfChunk {
    fn read(buffer: Vec<u8>) -> Self {
        let mut m = unsafe { core::mem::MaybeUninit::<Self>::uninit().assume_init() };
        m.decode_body(&buffer[0xC..]);
        m
    }
}

impl<T: BlfChunk + Sized + SerializableBlfChunk> ReadableBlfChunk for T {

}

pub trait TitleAndBuild {
    fn get_title() -> &'static str;

    fn get_build_string() -> &'static str;
}

pub trait ChunkFactory {
    fn decode(&self, signature: &chunk_signature, version: chunk_version, buffer: &[u8]) -> Result<Box<dyn DynamicBlfChunk>, &'static str>;
}