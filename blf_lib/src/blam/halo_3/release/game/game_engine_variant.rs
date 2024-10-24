use std::fmt::{Debug, Formatter};
use std::io::Cursor;
use std::mem::ManuallyDrop;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use blf_lib::blam::halo_3::release::game::game_engine_default::c_game_engine_base_variant;
use blf_lib::blam::halo_3::release::game::game_engine_slayer::c_game_engine_slayer_variant;
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;
use blf_lib_derive::PackedSerialize;
use serde::ser::SerializeStruct;
use crate::io::packed_decoding::PackedDecoder;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_variant {
    m_game_engine_index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    m_base_variant: Option<c_game_engine_base_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    m_slayer_variant: Option<c_game_engine_slayer_variant>,
}

impl PackedEncoder for c_game_variant {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        let mut result = self.m_game_engine_index.encode_packed(endian, packing);

        match self.m_game_engine_index {
            0 => {
                result.append(&mut self.m_base_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            2 => {
                result.append(&mut self.m_slayer_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            _ => {
                panic!("Tried to encode an unsupported game engine! ({})", self.m_game_engine_index);
            }
        }

        result
    }
}

impl PackedDecoder for c_game_variant {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>
    where
        Self: Sized
    {
        let mut game_variant = c_game_variant::default();
        game_variant.m_game_engine_index = u32::decode_packed(reader, endian, packing)?;

        match game_variant.m_game_engine_index {
            0 => {
                game_variant.m_base_variant = Some(c_game_engine_base_variant::decode_packed(reader, endian, packing)?)
            }
            2 => {
                game_variant.m_slayer_variant = Some(c_game_engine_slayer_variant::decode_packed(reader, endian, packing)?)
            }
            _ => {
                panic!("Tried to encode an unsupported game engine! ({})", game_variant.m_game_engine_index);
            }
        }

        Ok(game_variant)
    }
}