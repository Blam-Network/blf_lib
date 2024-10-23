pub(crate) mod halo3;

use std::fs::File;
use std::io::{Read, Seek};
use blf_lib::blf::s_blf_header;
pub use blf_lib_derivable::blf::chunks::*;


pub fn find_chunk<'a, T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(buffer: &Vec<u8>) -> Result<T, &'a str> {
    let mut offset: usize = 0;
    while offset < buffer.len() {
        let header = s_blf_header::decode(&buffer[offset..offset + (s_blf_header::size())]);

        if header.signature == T::get_signature() && header.version == T::get_version() {
            return Ok(T::read(buffer[offset..].to_vec(), Some(header)))
        }

        offset += header.chunk_size as usize;
    }

    Err("Could not find chunk")
}

pub fn find_chunk_in_file<T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(path: &str) -> Result<T, String> {
    let mut file = File::open(path).unwrap();
    let mut headerBytes = [0u8; s_blf_header::size()];
    let mut header: s_blf_header;

    while file.read_exact(&mut headerBytes).is_ok() {
        header = s_blf_header::decode(&headerBytes);
        if header.signature == T::get_signature() && header.version == T::get_version() {
            let mut body_bytes = vec![0u8; (header.chunk_size as usize) - s_blf_header::size()];
            file.read_exact(body_bytes.as_mut_slice()).unwrap();
            return Ok(T::read(body_bytes, Some(header)));
        }
        if header.chunk_size == 0 {
            break;
            }
        file.seek_relative((header.chunk_size - s_blf_header::size() as u32) as i64).unwrap();
    }
    Err(format!("{} Chunk not found!", T::get_signature().to_string()))
}