use std::ffi::c_char;
use std::fmt::Write;
use blf_lib::types::array::StaticArray;
use serde::{Deserializer, Serialize, Serializer};
use widestring::U16CString;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::{Packing, PACK1};
use std::io::Cursor;
use serde::de::Error;

pub fn to_string(chars: &[c_char]) -> String {
    let mut res = String::new();
    for char in chars {
        let copy: u8 = char.clone() as u8;
        if copy == 0 {
            break;
        }
        res.write_char(char::from(copy)).unwrap();
    }
    res
}

pub fn from_string_with_length(string: String, length: usize) -> Vec<c_char> {
    let mut vec = from_string(string);

    vec.resize(length, 0);

    vec
}

pub fn from_string(string: String) -> Vec<c_char> {
    let mut vec = Vec::new();

    let bytes = string.as_bytes();

    if string.len() != bytes.len() {
        panic!("Invalid string.");
    }

    for i in 0..bytes.len() {
        vec.push(bytes[i] as c_char);
    }

    vec
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct StaticWcharString<const N: usize> {
    buf: StaticArray<u16, N>,
}

impl<const N: usize> StaticWcharString<N> {
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
            .filter(|&c| c != '\u{0000}')
            .collect::<String>()
    }
}

impl<const N: usize> PackedEncoder for StaticWcharString<N> {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        let mut out = Vec::<u8>::with_capacity(N);
        self.buf.get().iter().for_each(|&wchar| {out.append(&mut (wchar).encode_packed(endian, PACK1));});
        out
    }
}

impl<const N: usize> PackedDecoder for StaticWcharString<N> {
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

impl<const N: usize> Serialize for StaticWcharString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&format!("{}", self.get_string()))
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for StaticWcharString<N> {
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

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct StaticString<const N: usize> {
    buf: [u8; N],
}

impl<const N: usize> StaticString<N> {
    pub fn from_string(value: impl Into<String>) -> Result<Self, String> {
        let mut new = Self {
            buf: [0; N],
        };

        let result = new.set_string(&value.into());
        if result.is_ok() { Ok(new) }
        else { Err(result.unwrap_err()) }
    }

    pub fn set_string(&mut self, value: &String) -> Result<(), String> {
        let mut bytes = value.as_bytes();
        // if a null termination was provided at the end, chop it off
        if bytes.len() > 0 && bytes[bytes.len() - 1] == 0 {
            bytes = &bytes[0..bytes.len() - 1];
        }
        if bytes.len() > N {
            return Err(format!("String \"{value}\" too long ({} > {}) bytes", N, bytes.len()));
        }
        self.buf.fill(0);
        self.buf[..bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    pub fn get_string(&self) -> String {
        let null_index = self.buf.iter().position(|c|c == &0u8).unwrap_or(N);
        String::from_utf8(self.buf.as_slice()[0..null_index].to_vec()).unwrap()
    }
}

impl<const N: usize> PackedEncoder for StaticString<N> {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        self.buf.encode_packed(endian, packing)
    }
}

impl<const N: usize> PackedDecoder for StaticString<N> {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String> {
        Ok(Self {
            buf: PackedDecoder::decode_packed(reader, endian, packing)?,
        })
    }
}

impl<const N: usize> Default for StaticString<N>  {
    fn default() -> Self {
        Self{
            buf: [0; N],
        }
    }
}

impl<const N: usize> Serialize for StaticString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&format!("{}", self.get_string()))
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for StaticString<N> {
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