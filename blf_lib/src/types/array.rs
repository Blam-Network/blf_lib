use std::io::Cursor;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;
use serde_big_array::BigArray;

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, Encode, Decode)]
pub struct Array<E: Default + Copy + PackedDecoder + PackedEncoder + Serialize + for <'de2> Deserialize<'de2> + 'static, const N: usize> {
    #[serde(with = "BigArray")]
    _data: [E; N]
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder + Serialize + for <'de2> Deserialize<'de2> + 'static, const N: usize> Array<E, N> {
    pub fn get(&self) -> &[E; N] {
         &self._data
    }

    pub fn get_mut(&mut self) -> &mut [E; N] {
        &mut self._data
    }

    pub fn from_slice(slice: &[E]) -> Result<Self, String> {
        if slice.len() != N {
            return Err(format!("Expected {N} elements but got {}.", slice.len()));
        }
        let mut new = Self {
            _data: [E::default(); N],
        };
        new._data.copy_from_slice(slice);
        Ok(new)
    }
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder + Serialize + for <'de2> Deserialize<'de2> + 'static, const N: usize> Default for Array<E, N> {
    fn default() -> Self {
        Self {
            _data: [E::default(); N]
        }
    }
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder + Serialize + for <'de2> Deserialize<'de2> + 'static, const N: usize> PackedDecoder for Array<E, N> {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>
    where
        Self: Sized
    {
        let mut vector = [E::default(); N];

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