use std::io::Cursor;
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Array<E: Default + Copy + PackedDecoder + PackedEncoder, const N: usize> {
    _data: [E; N]
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder, const N: usize> Array<E, N> {
    pub fn get(&mut self) -> &[E; N] {
         &self._data
    }
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder, const N: usize> Default for Array<E, N> {
    fn default() -> Self {
        Self {
            _data: [E::default(); N]
        }
    }
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder, const N: usize> PackedDecoder for Array<E, N> {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>
    where
        Self: Sized
    {
        let mut vector = Vec::<E>::with_capacity(N);

        for i in 0..N {
            vector.push(E::decode_packed(reader, endian, packing)?);
        }

        Ok(Self {
            _data: <[E; N]>::try_from(vector.as_slice()).unwrap()
        })
    }
}

impl<E: Default + Copy + PackedDecoder + PackedEncoder, const N: usize> PackedEncoder for Array<E, N> {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        self._data.iter().for_each(|&e| {
            buffer.append(&mut PackedEncoder::encode_packed(&e, endian, packing.clone()));
        });

        buffer
    }
}