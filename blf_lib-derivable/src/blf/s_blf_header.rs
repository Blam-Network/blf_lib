#![allow(non_camel_case_types)]

use std::ffi::c_char;
use crate::types::chunk_signature::chunk_signature;
use crate::types::chunk_version::chunk_version;

#[derive(Default)]
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
        let mut result = Vec::<u8>::with_capacity(Self::size());
        result.extend_from_slice(&self.signature.as_bytes());
        result.extend_from_slice(&self.chunk_size.to_be_bytes());
        result.extend_from_slice(&self.version.major.to_be_bytes());
        result.extend_from_slice(&self.version.minor.to_be_bytes());
        result
    }

    pub fn decode(data: &[u8]) -> s_blf_header {
        assert_eq!(data.len(), Self::size());

        let signature = chunk_signature::new([
            data[0] as c_char,
            data[1] as c_char,
            data[2] as c_char,
            data[3] as c_char,
        ]);

        let chunk_size = u32::from_be_bytes(data[4..8].try_into().unwrap());

        let version = chunk_version {
            major: u16::from_be_bytes(data[8..10].try_into().unwrap()),
            minor: u16::from_be_bytes(data[10..12].try_into().unwrap()),
        };

        Self {
            signature,
            chunk_size,
            version
        }
    }

    pub const fn size() -> usize {
        0xC
    }
}