use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::io::Cursor;
use serde::{Deserializer, Serialize, Serializer};
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::{Packing, PACK1};
use crate::io::packed_encoding::PackedEncoder;
use widestring::U16CString;
use crate::types::array::StaticArray;
use serde::de::Error;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct ByteLimitedWcharString<const N: usize> {
    buf: StaticArray<u16, N>,
}

impl<const N: usize> ByteLimitedWcharString<N> {
    pub fn from_string(value: &String) -> Result<Self, String> {
        let mut new = Self {
            buf: StaticArray::default()
        };

        let result = new.set_string(value);
        if result.is_ok() { Ok(new) }
        else { Err(result.unwrap_err()) }
    }

    pub fn set_string(&mut self, value: &String) -> Result<(), String> {
        let u16Str = U16CString::from_str(value).unwrap();
        let u16s = u16Str.as_slice();
        if u16s.len() > N {
            return Err(format!("String too long ({} > {}) bytes", N, u16s.len()));
        }
        let buf = self.buf.get_mut();
        buf.fill(0);
        buf[0..u16s.len()].copy_from_slice(&u16s);
        Ok(())
    }

    pub fn get_string(&self) -> String {
         decode_utf16(self.buf.get().iter().cloned())
            .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
            .filter(|&c| c as u8 != 0x00)
            .collect::<String>()
    }
}

impl<const N: usize> PackedEncoder for ByteLimitedWcharString<N> {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        let mut out = Vec::<u8>::with_capacity(N);
        self.buf.get().iter().for_each(|&wchar| {out.append(&mut (wchar).encode_packed(endian, PACK1));});
        out
    }
}

impl<const N: usize> PackedDecoder for ByteLimitedWcharString<N> {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String> {
        let mut buf = [u16::default(); N];
        for i in 0..N {
            buf[i] = u16::decode_packed(reader, endian, PACK1)?;
        }
        Ok(Self {
            buf: StaticArray::from_slice(&buf).unwrap(),
        })
    }
}

impl<const N: usize> Serialize for ByteLimitedWcharString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&format!("{}", self.get_string()))
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for ByteLimitedWcharString<N> {
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