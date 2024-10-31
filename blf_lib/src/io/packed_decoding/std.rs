use std::io::{Cursor, Read};
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;
use crate::io::packed_decoding::seek_pad;

impl PackedDecoder for u8 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 1];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(match endian {
            Endianness::Little => { u8::from_le_bytes(bytes) }
            Endianness::Big => { u8::from_be_bytes(bytes) }
        })
    }
}

impl PackedDecoder for i8 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 1];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(match endian {
            Endianness::Little => { i8::from_le_bytes(bytes) }
            Endianness::Big => { i8::from_be_bytes(bytes) }
        })
    }
}

impl PackedDecoder for bool {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 1];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(match endian {
            Endianness::Little => { u8::from_le_bytes(bytes) != 0 }
            Endianness::Big => { u8::from_be_bytes(bytes) != 0 }
        })
    }
}

impl PackedDecoder for u16 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 2];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(match endian {
            Endianness::Little => { u16::from_le_bytes(bytes) }
            Endianness::Big => { u16::from_be_bytes(bytes) }
        })
    }
}

impl PackedDecoder for i16 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 2];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(match endian {
            Endianness::Little => { i16::from_le_bytes(bytes) }
            Endianness::Big => { i16::from_be_bytes(bytes) }
        })
    }
}

impl PackedDecoder for u32 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(match endian {
            Endianness::Little => { u32::from_le_bytes(bytes) }
            Endianness::Big => { u32::from_be_bytes(bytes) }
        })
    }
}

impl PackedDecoder for i32 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(match endian {
            Endianness::Little => { i32::from_le_bytes(bytes) }
            Endianness::Big => { i32::from_be_bytes(bytes) }
        })
    }
}

impl PackedDecoder for u64 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(match endian {
            Endianness::Little => { u64::from_le_bytes(bytes) }
            Endianness::Big => { u64::from_be_bytes(bytes) }
        })
    }
}

impl PackedDecoder for i64 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(match endian {
            Endianness::Little => { i64::from_le_bytes(bytes) }
            Endianness::Big => { i64::from_be_bytes(bytes) }
        })
    }
}

impl PackedDecoder for f32 {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(match endian {
            Endianness::Little => { f32::from_le_bytes(bytes) }
            Endianness::Big => { f32::from_be_bytes(bytes) }
        })
    }
}

impl<const N: usize> PackedDecoder for [u8; N] {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; N];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;
        Ok(bytes)
    }
}

impl<const N: usize> PackedDecoder for [i8; N] {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; N];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;
        Ok(bytes.map(|byte|byte as i8))
    }
}

impl PackedDecoder for String {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String> {
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer).map_err(|_| "Failed to read bytes.")?;
        Ok(String::from_utf8(buffer).unwrap())
    }
}

impl<T: PackedDecoder> PackedDecoder for Vec<T> {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String> {
        let mut vector = Vec::<T>::new();
        let mut element_buffer = Vec::<u8>::new();
        reader.read_to_end(&mut element_buffer).map_err(|_| "Failed to read bytes.")?;
        let mut element_cursor = Cursor::new(element_buffer.as_slice());
        // since no size is provided, we assume we're reading until the end.
        let mut decoded: Result<T, String>;
        loop {
            let decoded = T::decode_packed(&mut element_cursor, endian, packing);
            if decoded.is_err() {
                break;
            }
            vector.push(decoded.unwrap());
        }
        Ok(vector)
    }
}