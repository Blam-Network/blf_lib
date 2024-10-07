use std::ffi::c_char;

pub trait Serializable {
    fn encode(&self) -> &[u8];
    fn decode(buffer: &[u8]) -> Self;
}

pub trait BlfChunk {
    fn get_signature() -> [c_char; 4];
    fn get_version() -> [u16; 2];
}

pub trait ChunkFactory {
    fn decode_chunk(signature: &[c_char; 4], major_version: u16, minor_version: u16, buffer: &[u8]) -> Result<impl BlfChunk + Serializable, &'static str>;

    fn get_title() -> &'static str;

    fn get_build_string() -> &'static str;
}