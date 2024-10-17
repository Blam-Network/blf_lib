use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use crate::blf_chunk;

blf_chunk!(
    #[Signature("_eof")]
    #[Version(1.1)]
    #[Size(0x5)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_end_of_file
    {
        pub file_size: u32,
        pub authentication_type: e_blf_file_authentication_type,
    }
);

impl s_blf_chunk_end_of_file {
    // automagically called for _eof chunks by BytePackedEncodeedSerializable derive
    fn update_eof(&mut self, written_bytes: &Vec<u8>) {
        self.file_size = written_bytes.len() as u32;
    }
}

#[derive(Encode, Decode, Debug, Clone, Copy, Default, PartialEq, PackedSerialize, Serialize, Deserialize)]
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

    fn before_write(&mut self, previous_chunks: Vec<u8>) {
        self.file_size = previous_chunks.len() as u32;
    }
}