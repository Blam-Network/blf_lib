use std::ffi::CStr;
use bincode::{Decode, Encode};
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;
use blf_lib_derive::PackedSerialize;
use crate::io::packed_encoding::PackedEncoder;

#[derive(PartialEq, Debug, Clone, Encode, Decode, Copy, PackedSerialize)]
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