use std::ffi::CStr;
use std::io::Cursor;
use serde::{Deserializer, Serialize, Serializer};
use serde::de::Error;
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;
use crate::io::packed_encoding::PackedEncoder;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FixedSizeUTF8String<const N: usize> {
    buf: [u8; N],
}

impl<const N: usize> FixedSizeUTF8String<N> {
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

impl<const N: usize> PackedEncoder for FixedSizeUTF8String<N> {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        self.buf.encode_packed(endian, packing)
    }
}

impl<const N: usize> PackedDecoder for FixedSizeUTF8String<N> {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String> {
        Ok(Self {
            buf: PackedDecoder::decode_packed(reader, endian, packing)?,
        })
    }
}

impl<const N: usize> Default for FixedSizeUTF8String<N>  {
    fn default() -> Self {
        Self{
            buf: [0; N],
        }
    }
}

impl<const N: usize> Serialize for FixedSizeUTF8String<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&format!("{}", self.get_string()))
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for FixedSizeUTF8String<N> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let res = Self::from_string(&s);
        if res.is_err() {
            Err(D::Error::custom(res.unwrap_err()))
        } else {
            Ok(res.unwrap())
        }
    }
}