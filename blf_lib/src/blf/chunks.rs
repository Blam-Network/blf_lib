pub(crate) mod halo3;
pub(crate) mod halo3odst;

use std::fs::File;
use std::io::{Read, Seek};
use serde::Deserialize;
use blf_lib::blf::s_blf_header;
pub use blf_lib_derivable::blf::chunks::*;


pub fn find_chunk<'a, T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(buffer: Vec<u8>) -> Result<T, &'a str> {
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

pub fn find_chunk_in_file<T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(path: impl Into<String>) -> Result<T, String> {
    let mut file = File::open(path.into()).unwrap();
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

pub fn search_for_chunk_in_file<T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(path: &str) -> Option<T> {
    let mut fileBytes = Vec::<u8>::new();
    File::open(path).unwrap().read_to_end(&mut fileBytes).unwrap();

    for i in 0..(fileBytes.len() - 0xC) {
        let header_bytes = &fileBytes.as_slice()[i..i+0xC];
        let header = s_blf_header::decode(&header_bytes);

        if header.signature == T::get_signature() && header.version == T::get_version() {
            let body_bytes = fileBytes.as_slice()[i+0xC..i+header.chunk_size as usize].to_vec();
            return Some(T::read(body_bytes, Some(header)));
        }
    }

    None
}

pub fn read_chunk_json<T: BlfChunk + for<'d> Deserialize<'d>>(path: &str) -> Result<T, String> {
    let mut file = File::open(path).unwrap();
    let parsed = serde_json::from_reader(&mut file);
    if parsed.is_err() {
        return Err(parsed.err().unwrap().to_string());
    }
    Ok(parsed.unwrap())
}