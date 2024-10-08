use bincode::Encode;
use blf_lib_derive::{TestSize, UnpackedSerializable};

use crate::types::chunk_signature::chunk_signature;

#[repr(C, packed)]
#[derive(Default, UnpackedSerializable, TestSize)]
#[Size(0xC)]
#[LittleEndian]
pub struct s_blf_header
{
    pub signature: chunk_signature,
    pub chunk_size: u32,
    pub major_version: u16,
    pub minor_version: u16,
}

impl s_blf_header {
    pub fn setup(signature: chunk_signature, chunk_size: u32, version: [u16; 2]) -> s_blf_header {
        s_blf_header {
            signature,
            chunk_size,
            major_version: version[0],
            minor_version: version[1],
        }
    }
}