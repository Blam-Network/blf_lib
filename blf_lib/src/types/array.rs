use std::io::Cursor;
use std::ops::Index;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeSeq;
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib::types::byte_limited_wchar_string::ByteLimitedWcharString;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;

#[derive(PartialEq, Debug, Clone)]
pub struct Array<E: 'static, const N: usize> {
    _data: Vec<E> // 1984
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder + Serialize + for <'de2> Deserialize<'de2> + 'static, const N: usize> Array<E, N> {
    pub fn get(&self) -> &Vec<E> {
         &self._data
    }

    pub fn get_mut(&mut self) -> &mut Vec<E> {
        &mut self._data
    }

    pub fn from_slice(slice: &[E]) -> Result<Self, String> {
        if slice.len() != N {
            return Err(format!("Expected {N} elements but got {}.", slice.len()));
        }
        let mut new = Self {
            _data: vec![E::default(); N],
        };
        new._data.copy_from_slice(slice);
        Ok(new)
    }
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder + Serialize + for <'de2> Deserialize<'de2> + 'static, const N: usize> Default for Array<E, N> {
    fn default() -> Self {
        Self {
            _data: vec![E::default(); N]
        }
    }
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder + Serialize + for <'de2> Deserialize<'de2> + 'static, const N: usize> PackedDecoder for Array<E, N> {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>
    where
        Self: Sized
    {
        let mut vector = vec![E::default(); N];

        for i in 0..N {
            vector[i] = E::decode_packed(reader, endian, packing)?;
        }

        Ok(Self {
            _data: vector,
        })
    }
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder + Serialize + for <'de2> Deserialize<'de2> + 'static, const N: usize> PackedEncoder for Array<E, N> {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        self._data.iter().for_each(|&e| {
            buffer.append(&mut PackedEncoder::encode_packed(&e, endian, packing.clone()));
        });

        buffer
    }
}

impl<E, const N: usize> Index<usize> for Array<E, N> {
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        &self._data[index]
    }
}

impl<E: Serialize, const N: usize> Serialize for Array<E, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        self._data.serialize(serializer)
    }
}

impl<'de, E: Deserialize<'de>, const N: usize> serde::Deserialize<'de> for Array<E, N> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self {
            _data: Vec::<E>::deserialize(d)?
        })
    }
}