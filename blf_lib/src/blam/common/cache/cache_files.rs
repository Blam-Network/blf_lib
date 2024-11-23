use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};
use binrw::{BinRead, BinReaderExt, BinWrite};
use libc::c_char;
use blf_lib::types::array::StaticArray;
use crate::types::c_string::StaticString;
use crate::types::c_string::to_string;

#[derive(Debug, BinRead, BinWrite)]
#[brw(big)]
pub struct s_cache_file_header_v11 {
    head: [c_char; 4], // "head"
    pub version: u32, // 11
    pub size: u32,
    unknown1: StaticArray<u8, 0x180>,
    pub map_name: StaticString<0x20>,
    pub map_language: u32,
    unknown2: StaticArray<u8, 0x1BC>,
    pub rsa_signature: StaticArray<u8, 0x100>,
    unknown3: StaticArray<u8, 0x2B90>,
    foot: [c_char; 4], // "foot"
}

impl s_cache_file_header_v11 {
    pub fn read(path: String) -> Result<Self, Box<dyn Error>> {
        let mut input_file = File::open(path)?;
        let mut buffer = [0u8; 0x3000];
        input_file.read_exact(buffer.as_mut_slice()).expect("Failed to read cache file.");
        let mut reader = Cursor::new(buffer);
        let cache_file: s_cache_file_header_v11 = reader.read_ne()?;

        if to_string(&cache_file.head) != "head" {
            return Err("Invalid cache file: Missing head.".into())
        }

        if to_string(&cache_file.foot) != "foot" {
            return Err("Invalid cache file: Missing foot.".into())
        }

        if cache_file.version != 11 {
            return Err(format!("Invalid cache file: Expected version {}, got {}", 11, cache_file.version).into());
        }

        Ok(cache_file)
    }
}