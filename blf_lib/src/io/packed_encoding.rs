use std::mem;
use std::ops::{Rem, Sub};
use crate::io::endian::Endianness;
use crate::io::packing::Packing;

pub trait PackedEncoder {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8>;
}

impl PackedEncoder for u8 {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        match endian {
            Endianness::Little => { packing.create_packed_buffer_from_slice(self.to_le_bytes().as_slice()) }
            Endianness::Big => { packing.create_packed_buffer_from_slice(self.to_be_bytes().as_slice()) }
        }
    }
}

impl PackedEncoder for u16 {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        match endian {
            Endianness::Little => { packing.create_packed_buffer_from_slice(self.to_le_bytes().as_slice()) }
            Endianness::Big => { packing.create_packed_buffer_from_slice(self.to_be_bytes().as_slice()) }
        }
    }
}

impl PackedEncoder for u32 {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        match endian {
            Endianness::Little => { packing.create_packed_buffer_from_slice(self.to_le_bytes().as_slice()) }
            Endianness::Big => { packing.create_packed_buffer_from_slice(self.to_be_bytes().as_slice()) }
        }
    }
}