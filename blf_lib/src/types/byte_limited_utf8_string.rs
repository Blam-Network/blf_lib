use std::ffi::CStr;
use std::io::Cursor;
use bincode::{Decode, Encode};
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;
use crate::io::packed_encoding::PackedEncoder;

#[derive(PartialEq, Debug, Clone, Encode, Decode, Copy)]
pub struct ByteLimitedUTF8String<const N: usize> {
    buf: [u8; N],
}

impl<const N: usize> ByteLimitedUTF8String<N> {
    pub fn from_string(value: &String) -> Result<Self, String> {
        let mut new = Self {
            buf: [0; N],
        };

        let result = new.set_string(value);
        if result.is_ok() { Ok(new) }
        else { Err(result.unwrap_err()) }
    }

    pub fn set_string(&mut self, value: &String) -> Result<(), String> {
        let bytes = value.as_bytes();
        if bytes.len() > N {
            return Err(format!("String too long ({} > {}) bytes", N, bytes.len()));
        }
        self.buf.fill(0);
        self.buf[..bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    pub fn get_string(&self) -> String {
        CStr::from_bytes_until_nul(self.buf.as_slice()).unwrap().to_str().unwrap().to_string()
    }
}

impl<const N: usize> PackedEncoder for ByteLimitedUTF8String<N> {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        self.buf.encode_packed(endian, packing)
    }
}

impl<const N: usize> PackedDecoder for ByteLimitedUTF8String<N> {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Self {
        Self {
            buf: PackedDecoder::decode_packed(reader, endian, packing),
        }
    }
}