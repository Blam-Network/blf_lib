use std::ffi::c_char;
use bincode::{Decode, Encode};
use blf_lib_derive::{BlfChunk, TestSize, UnpackedSerializable};

#[derive(BlfChunk, UnpackedSerializable, TestSize, Debug)]
#[Signature("_eof")]
#[Version(1.1)]
#[Size(0x5)]
#[BigEndian]
#[Pack(1)]
pub struct s_blf_chunk_end_of_file
{
    pub file_size: u32,
    pub authentication_type: e_blf_file_authentication_type,
}

#[derive(Encode, Decode, Debug, Clone, Copy)]
pub struct e_blf_file_authentication_type {
    value: u8,
}

pub const _blf_file_authentication_type_none: e_blf_file_authentication_type = e_blf_file_authentication_type { value: 0 };
pub const _blf_file_authentication_type_crc: e_blf_file_authentication_type = e_blf_file_authentication_type { value: 1 };
pub const _blf_file_authentication_type_sha1: e_blf_file_authentication_type = e_blf_file_authentication_type { value: 2 };
pub const _blf_file_authentication_type_rsa: e_blf_file_authentication_type = e_blf_file_authentication_type { value: 2 };


impl s_blf_chunk_end_of_file {
    pub fn new(file_size: u32, authentication_type: e_blf_file_authentication_type) -> s_blf_chunk_end_of_file {
        s_blf_chunk_end_of_file {
            file_size,
            authentication_type,
        }
    }
}

impl Default for s_blf_chunk_end_of_file {
    fn default() -> Self {
        Self {
            file_size: 0,
            authentication_type: _blf_file_authentication_type_none
        }
    }
}

