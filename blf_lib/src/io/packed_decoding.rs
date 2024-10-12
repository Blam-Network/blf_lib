use crate::io::endian::Endianness;
use crate::io::packing::Packing;

pub trait DecodePacked {
    fn decode_packed<T>(bytes: &[u8], endian: Endianness, packing: Packing) -> T;
}