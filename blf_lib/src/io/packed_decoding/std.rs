use std::ffi::CStr;
use std::io::{Cursor, Read, Seek};
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;

impl PackedDecoder for u8 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Self {
        let mut bytes = [0u8; 1];
        reader.read_exact(&mut bytes).unwrap();
        reader.seek_relative(packing.get_padding(bytes.len()) as i64).unwrap();

        match endian {
            Endianness::Little => { u8::from_le_bytes(bytes) }
            Endianness::Big => { u8::from_be_bytes(bytes) }
        }
    }
}

impl PackedDecoder for u16 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Self {
        let mut bytes = [0u8; 2];
        reader.read_exact(&mut bytes).unwrap();
        reader.seek_relative(packing.get_padding(bytes.len()) as i64).unwrap();

        match endian {
            Endianness::Little => { u16::from_le_bytes(bytes) }
            Endianness::Big => { u16::from_be_bytes(bytes) }
        }
    }
}

impl PackedDecoder for u32 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Self {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes).unwrap();
        reader.seek_relative(packing.get_padding(bytes.len()) as i64).unwrap();

        match endian {
            Endianness::Little => { u32::from_le_bytes(bytes) }
            Endianness::Big => { u32::from_be_bytes(bytes) }
        }
    }
}

impl<const N: usize> PackedDecoder for [u8; N] {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Self {
        let mut bytes = [0u8; N];
        reader.read_exact(&mut bytes).unwrap();
        reader.seek_relative(packing.get_padding(bytes.len()) as i64).unwrap();
        bytes
    }
}

impl<const N: usize> PackedDecoder for [i8; N] {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Self {
        let mut bytes = [0u8; N];
        reader.read_exact(&mut bytes).unwrap();
        reader.seek_relative(packing.get_padding(bytes.len()) as i64).unwrap();
        bytes.map(|byte|byte as i8)
    }
}

impl PackedDecoder for String {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Self {
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer).unwrap();
        CStr::from_bytes_until_nul(buffer.as_slice()).unwrap().to_str().unwrap().to_string()
    }
}

impl<T: PackedDecoder> PackedDecoder for Vec<T> {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Self {
        let mut vector = Vec::<T>::new();
        vector.push(T::decode_packed(reader, endian, packing));
        vector
    }
}