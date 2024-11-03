use std::fs::File;
use std::io::{Cursor, Read};
use libc::c_char;
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::PACK1;
use blf_lib_derive::{PackedSerialize};
use crate::io::packed_decoding::PackedDecoder;
use crate::types::c_string::StaticString;
use crate::types::c_string::to_string;

#[derive(PackedSerialize, Debug)]
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
    pub fn read(path: String) -> Result<Self, String> {
        let mut input_file = File::open(path).unwrap();
        let mut buffer = [0u8; 0x3000];
        input_file.read_exact(buffer.as_mut_slice()).expect("Failed to read cache file.");
        let cache_file = <Self as PackedDecoder>::decode_packed(&mut Cursor::new(&buffer), Endianness::Big, PACK1).expect("Failed to decode cache file.");

        if to_string(&cache_file.head) != "head" {
            return Err("Invalid cache file: Missing head.".to_string())
        }

        if to_string(&cache_file.foot) != "foot" {
            return Err("Invalid cache file: Missing foot.".to_string())
        }

        if cache_file.version != 11 {
            return Err(format!("Invalid cache file: Expected version {}, got {}", 11, cache_file.version));
        }

        Ok(cache_file)
    }
}