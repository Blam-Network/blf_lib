use blf_lib_derivable::blf::chunks::{BlfChunk, Serializable};

mod chunks;

pub mod s_blf_header;

pub mod versions;

pub fn write_blf_chunk<T: BlfChunk + Serializable>(chunk: &T) -> Vec<u8> {
    let mut encoded_chunk = chunk.encode_chunk();
    let header = crate::blf::s_blf_header::s_blf_header {
        signature: T::get_signature(),
        major_version: T::get_version().major,
        minor_version: T::get_version().minor,
        chunk_size: encoded_chunk.len() as u32,
    };

    let mut encoded_header = header.encode_chunk();
    encoded_header.append(&mut encoded_chunk);

    encoded_header
}