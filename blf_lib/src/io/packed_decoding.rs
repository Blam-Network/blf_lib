mod std;

use ::std::io::{Cursor, Seek};
use crate::io::endian::Endianness;
use crate::io::packing::Packing;

pub trait PackedDecoder {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String> where Self: Sized;
}

pub(crate) fn seek_pad(reader: &mut Cursor<&[u8]>, bytes: &[u8], packing: Packing) -> Result<(), String> {
    let result = reader.seek_relative(packing.get_padding(bytes.len()) as i64);
    if result.is_ok() { Ok(()) }
    else { Err("seek_relative failed".to_string()) }
}