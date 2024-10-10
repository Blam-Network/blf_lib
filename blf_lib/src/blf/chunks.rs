pub(crate) mod halo3;

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
pub use blf_lib_derivable::blf::chunks::*;
use blf_lib_derivable::blf::s_blf_header::s_blf_header;
use crate::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_author;

pub fn find_chunk<'a, T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(buffer: Vec<u8>) -> Result<T, &'a str> {
    let mut offset: usize = 0;
    while offset < buffer.len() {
        let header = s_blf_header::decode(&buffer[offset..offset + (s_blf_header::size() as usize)]);

        if header.signature == T::get_signature() && header.version == T::get_version() {
            return Ok(T::read(buffer[offset..].to_vec(), true))
        }

        offset += (header.chunk_size as usize) + s_blf_header::size();
    }

    Err("Could not find chunk")
}

pub fn find_chunk_in_file<T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(path: &str) -> Result<T, String> {
    let mut file = File::open(path).unwrap();
    let mut headerBytes = [0u8; s_blf_header::size()];
    let mut header: s_blf_header;

    while (file.read_exact(&mut headerBytes).is_ok()) {
        header = s_blf_header::decode(&headerBytes);
        if header.signature == T::get_signature() && header.version == T::get_version() {
            let body_bytes = Vec::with_capacity(header.chunk_size as usize);
            return Ok(T::read(body_bytes, false));
        }
        file.seek_relative(header.chunk_size as i64).unwrap();
    }
    Err(format!("{} Chunk not found!", T::get_signature().to_string()))
}