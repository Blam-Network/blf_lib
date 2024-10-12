use crate::io::endian::Endianness;
use crate::io::packing::Packing;

pub trait PackedEncoder {
    fn encode_packed(&self, endian: Endianness, packing: Packing, position: usize) -> Vec<u8>;
}

// impl PackedEncoder for u32 {
//     fn encode_packed(&self, endian: Endianness, packing: Packing, position: usize) -> Vec<u8> {
//         Vec<>
//     }
// }