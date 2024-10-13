pub mod std;
pub mod blf;

use crate::io::endian::Endianness;
use crate::io::packing::Packing;

pub trait PackedEncoder {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8>;
}
