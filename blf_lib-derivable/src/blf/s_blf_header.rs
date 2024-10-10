use bincode::{Decode, Encode};

use crate::types::chunk_signature::chunk_signature;
use crate::types::chunk_version::chunk_version;

#[derive(Default, Encode, Decode)]
pub struct s_blf_header
{
    pub signature: chunk_signature,
    pub chunk_size: u32,
    pub version: chunk_version,
}

impl s_blf_header {
    pub fn setup(signature: chunk_signature, chunk_size: u32, version: chunk_version) -> s_blf_header {
        s_blf_header {
            signature,
            chunk_size,
            version
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let bincode_config = bincode::config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();

        bincode::encode_to_vec(self, bincode_config).unwrap()
    }

    pub fn decode(data: &[u8]) -> s_blf_header {
        let bincode_config = bincode::config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();

        bincode::decode_from_slice(data, bincode_config).unwrap().0
    }

    pub const fn size() -> usize {
        0xC
    }
}