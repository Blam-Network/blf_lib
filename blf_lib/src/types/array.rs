use std::io::{Cursor, Read, Seek, Write};
use std::ops::Index;
use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;

#[derive(PartialEq, Debug)]
pub struct StaticArray<E: 'static, const N: usize> {
    _data: Vec<E> // 1984
}

impl<'a, E: BinRead<Args<'a> = ()> + BinWrite + 'static, const N: usize> BinRead for StaticArray<E, N> {
    type Args<'b> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        let mut data = Vec::with_capacity(N);

        // Manually read each element in the array with options
        for _ in 0..N {
            let element = E::read_options(reader, endian, ())?; // Read each element using BinRead for E with options
            data.push(element);
        }

        Ok(StaticArray { _data: data })
    }
}

impl<'a, E: BinRead + BinWrite<Args<'a> = ()> + 'static, const N: usize> BinWrite for StaticArray<E, N> {
    type Args<'b> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, _args: Self::Args<'_>) -> BinResult<()> {
        // Manually write each element in the array with options
        for item in &self._data {
            item.write_options(writer, endian, ())?; // Write each element using BinWrite for E with options
        }
        Ok(())
    }
}

impl<E: Clone, const N: usize> Clone for StaticArray<E, N> {
    fn clone(&self) -> Self {
        StaticArray {
            _data: self._data.clone(), // Clone the vector
        }
    }
}

impl<E: Default + BinRead + BinWrite + Clone, const N: usize> StaticArray<E, N> {
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
        new._data.clone_from_slice(slice);
        Ok(new)
    }
}

impl<E: Default + BinRead + BinWrite + Clone, const N: usize> Default for StaticArray<E, N> {
    fn default() -> Self {
        Self {
            _data: vec![E::default(); N]
        }
    }
}

impl<E: Default + PackedDecoder + BinRead + BinWrite + Clone + 'static, const N: usize> PackedDecoder for StaticArray<E, N> {
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

impl<E: Default + PackedEncoder + BinRead + BinWrite + Clone + 'static, const N: usize> PackedEncoder for StaticArray<E, N> {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        self._data.iter().for_each(|e| {
            buffer.append(&mut PackedEncoder::encode_packed(e, endian, packing.clone()));
        });

        buffer
    }
}

impl<E: BinRead + BinWrite + Clone, const N: usize> Index<usize> for StaticArray<E, N> {
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        &self._data[index]
    }
}

impl<E: Serialize + BinRead + BinWrite + Clone, const N: usize> Serialize for StaticArray<E, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        self._data.serialize(serializer)
    }
}

impl<'de, E: Deserialize<'de> + BinRead + BinWrite + Clone, const N: usize> serde::Deserialize<'de> for StaticArray<E, N> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self {
            _data: Vec::<E>::deserialize(d)?
        })
    }
}