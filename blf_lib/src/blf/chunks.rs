pub(crate) mod halo3;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
pub use blf_lib_derivable::blf::chunks::*;
use blf_lib_derivable::blf::s_blf_header::s_blf_header;
use crate::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_author;

pub fn find_chunk<'a, T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(buffer: Vec<u8>) -> Result<T, &'a str> {
    let mut offset: usize = 0;
    while offset < buffer.len() {
        let header = s_blf_header::decode(&buffer[offset..offset + (s_blf_header::size() as usize)]);

        if header.signature == T::get_signature() && header.version == T::get_version() {
            return Ok(T::read(buffer[offset..].to_vec()))
        }

        offset += (header.chunk_size + s_blf_header::size()) as usize ;
    }

    Err("Could not find chunk")
}

pub fn find_chunk_in_file<T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(path: &str) -> Result<T, &str> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).unwrap();
    find_chunk(buffer)
}