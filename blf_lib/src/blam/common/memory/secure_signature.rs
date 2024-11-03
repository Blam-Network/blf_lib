use std::error::Error;
use std::io::{Cursor, Read};
use serde::{Deserializer, Serialize, Serializer};
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;

#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct s_network_http_request_hash {
    pub data: [u8; 20]
}

impl Serialize for s_network_http_request_hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&format!("{}", hex::encode_upper(self.data)))
    }
}

impl<'de> serde::Deserialize<'de> for s_network_http_request_hash {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(hex::decode(String::deserialize(d)?).unwrap().try_into().unwrap())
    }
}

impl PackedEncoder for s_network_http_request_hash {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        Vec::<u8>::from(self.data)
    }
}

impl PackedDecoder for s_network_http_request_hash {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 20];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        crate::io::packed_decoding::seek_pad(reader, &bytes, packing)?;

        Ok(Self {
            data: bytes,
        })
    }
}

impl TryFrom<Vec<u8>> for s_network_http_request_hash {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let data: [u8; 20];
        data = value.try_into().unwrap();

        Ok(s_network_http_request_hash {
            data
        })
    }
}