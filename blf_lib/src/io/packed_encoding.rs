use std::mem;
use std::ops::{Rem, Sub};
use crate::io::endian::Endianness;
use crate::io::packing::Packing;

pub trait PackedEncoder {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8>;
}

impl PackedEncoder for u32 {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        let encoded_length = mem::size_of::<u32>();
        let padding_size = (packing - (encoded_length % packing)) % packing;
    }
}