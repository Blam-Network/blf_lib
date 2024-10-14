mod std;

use ::std::io::Cursor;
use crate::io::endian::Endianness;
use crate::io::packing::Packing;

pub trait PackedDecoder {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Self;
}