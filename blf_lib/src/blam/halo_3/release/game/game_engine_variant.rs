use std::io::Cursor;
use std::mem::ManuallyDrop;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_default::c_game_engine_base_variant;
use blf_lib::blam::halo_3::release::game::game_engine_slayer::c_game_engine_slayer_variant;
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;
use blf_lib_derive::PackedSerialize;
use crate::io::packed_decoding::PackedDecoder;

union game_engine_variant {
    m_base_variant: ManuallyDrop<c_game_engine_base_variant>,
    m_slayer_variant: ManuallyDrop<c_game_engine_slayer_variant>,
}

impl game_engine_variant {
    pub fn encode_packed(&self, game_engine_index: u32, endian: Endianness, packing: Packing) -> Vec<u8> {
        match game_engine_index {
            0 => {
                self.m_base_variant.encode_packed(endian, packing)
            }
            2 => {
                self.m_slayer_variant.encode_packed(endian, packing)
            }
            _ => {
                panic!("Tried to encode an unsupported game engine! ({game_engine_index})");
            }
        }
    }

    fn eq(&self, game_engine_index: u32, other: &Self) -> bool {
        match game_engine_index {
            0 => {
                self.m_base_variant == other.m_base_variant
            }
            2 => {
                self.m_slayer_variant == other.m_slayer_variant
            }
            _ => {
                panic!("Tried to compare an unsupported game engine! ({game_engine_index})");
            }
        }
    }
}

pub struct c_game_variant {
    m_game_engine_index: u32,
    m_game_engine_variant: game_engine_variant,
}

impl Default for c_game_variant {
    fn default() -> Self {
        Self {
            m_game_engine_index: 0,
            m_game_engine_variant: game_engine_variant {
                m_base_variant: ManuallyDrop::new(c_game_engine_base_variant::default())
            }
        }
    }
}

impl PartialEq for c_game_variant {
    fn eq(&self, other: &Self) -> bool {
        self.m_game_engine_index == other.m_game_engine_index
            &&  self.m_game_engine_variant.eq(self.m_game_engine_index, &other.m_game_engine_variant)
    }
}

impl PackedEncoder for c_game_variant {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        let mut result = self.m_game_engine_index.encode_packed(endian, packing);

        result.append(&mut self.m_game_engine_variant.encode_packed(
            self.m_game_engine_index,
            endian,
            packing
        ));

        result
    }
}

impl PackedDecoder for c_game_variant {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>
    where
        Self: Sized
    {
        todo!()
    }
}