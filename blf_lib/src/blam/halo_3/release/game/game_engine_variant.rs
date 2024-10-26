use std::fmt::{Debug};
use std::io::Cursor;
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::math::real_math::real_rectangle3d;
use blf_lib::blam::common::simulation::simulation_encoding::{simulation_read_quantized_position, simulation_write_quantized_position};
use blf_lib::blam::halo_3::release::game::game_engine_default::{c_game_engine_base_variant, c_game_engine_social_options};
use blf_lib::blam::halo_3::release::game::game_engine_slayer::c_game_engine_slayer_variant;
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::{Packing, PACK1};
use blf_lib::blam::halo_3::release::game::game_engine_assault::c_game_engine_assault_variant;
use blf_lib::blam::halo_3::release::game::game_engine_ctf::c_game_engine_ctf_variant;
use blf_lib::blam::halo_3::release::game::game_engine_infection::c_game_engine_infection_variant;
use blf_lib::blam::halo_3::release::game::game_engine_juggernaut::c_game_engine_juggernaut_variant;
use blf_lib::blam::halo_3::release::game::game_engine_king::c_game_engine_king_variant;
use blf_lib::blam::halo_3::release::game::game_engine_oddball::c_game_engine_oddball_variant;
use blf_lib::blam::halo_3::release::game::game_engine_sandbox::c_game_engine_sandbox_variant;
use blf_lib::blam::halo_3::release::game::game_engine_territories::c_game_engine_territories_variant;
use blf_lib::blam::halo_3::release::game::game_engine_vip::c_game_engine_vip_variant;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_object_identifier;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::TEST_BIT;
use crate::io::packed_decoding::PackedDecoder;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_variant {
    pub m_game_engine_index: u32,
    pub m_base_variant: c_game_engine_base_variant,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_ctf_variant: Option<c_game_engine_ctf_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_slayer_variant: Option<c_game_engine_slayer_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_oddball_variant: Option<c_game_engine_oddball_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_king_variant: Option<c_game_engine_king_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_sandbox_variant: Option<c_game_engine_sandbox_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_vip_variant: Option<c_game_engine_vip_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_juggernaut_variant: Option<c_game_engine_juggernaut_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_territories_variant: Option<c_game_engine_territories_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_assault_variant: Option<c_game_engine_assault_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_infection_variant: Option<c_game_engine_infection_variant>,
}

impl PackedEncoder for c_game_variant {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        let mut result = self.m_game_engine_index.encode_packed(endian, packing);

        result.append(&mut self.m_base_variant.encode_packed(endian, packing));

        match self.m_game_engine_index {
            0 => { }
            1 => {
                result.append(&mut self.m_ctf_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            2 => {
                result.append(&mut self.m_slayer_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            3 => {
                result.append(&mut self.m_oddball_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            4 => {
                result.append(&mut self.m_king_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            5 => {
                result.append(&mut self.m_sandbox_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            6 => {
                result.append(&mut self.m_vip_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            7 => {
                result.append(&mut self.m_juggernaut_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            8 => {
                result.append(&mut self.m_territories_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            9 => {
                result.append(&mut self.m_assault_variant.as_ref().unwrap().encode_packed(endian, packing))
            }
            10 => {
                result.append(&mut self.m_infection_variant.as_ref().unwrap().encode_packed(endian, packing))
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
        game_variant.m_base_variant = c_game_engine_base_variant::decode_packed(reader, endian, packing)?;

        match game_variant.m_game_engine_index {
            0 => { }
            1 => {
                game_variant.m_ctf_variant = Some(c_game_engine_ctf_variant::decode_packed(reader, endian, packing)?)
            }
            2 => {
                game_variant.m_slayer_variant = Some(c_game_engine_slayer_variant::decode_packed(reader, endian, packing)?)
            }
            3 => {
                game_variant.m_oddball_variant = Some(c_game_engine_oddball_variant::decode_packed(reader, endian, packing)?)
            }
            4 => {
                game_variant.m_king_variant = Some(c_game_engine_king_variant::decode_packed(reader, endian, packing)?)
            }
            5 => {
                game_variant.m_sandbox_variant = Some(c_game_engine_sandbox_variant::decode_packed(reader, endian, packing)?)
            }
            6 => {
                game_variant.m_vip_variant = Some(c_game_engine_vip_variant::decode_packed(reader, endian, packing)?)
            }
            7 => {
                game_variant.m_juggernaut_variant = Some(c_game_engine_juggernaut_variant::decode_packed(reader, endian, packing)?)
            }
            8 => {
                game_variant.m_territories_variant = Some(c_game_engine_territories_variant::decode_packed(reader, endian, packing)?)
            }
            9 => {
                game_variant.m_assault_variant = Some(c_game_engine_assault_variant::decode_packed(reader, endian, packing)?)
            }
            10 => {
                game_variant.m_infection_variant = Some(c_game_engine_infection_variant::decode_packed(reader, endian, packing)?)
            }
            _ => {
                panic!("Tried to encode an unsupported game engine! ({})", game_variant.m_game_engine_index);
            }
        }

        Ok(game_variant)
    }
}


impl c_game_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_integer(self.m_game_engine_index, 4);

        self.m_base_variant.encode(bitstream);

        match self.m_game_engine_index {
            0 => { }
            1 => { self.m_ctf_variant.as_ref().unwrap().encode(bitstream); }
            2 => { self.m_slayer_variant.as_ref().unwrap().encode(bitstream); }
            3 => { self.m_oddball_variant.as_ref().unwrap().encode(bitstream); }
            4 => { self.m_king_variant.as_ref().unwrap().encode(bitstream); }
            5 => { self.m_sandbox_variant.as_ref().unwrap().encode(bitstream); }
            6 => { self.m_vip_variant.as_ref().unwrap().encode(bitstream); }
            7 => { self.m_juggernaut_variant.as_ref().unwrap().encode(bitstream); }
            8 => { self.m_territories_variant.as_ref().unwrap().encode(bitstream); }
            9 => { self.m_assault_variant.as_ref().unwrap().encode(bitstream); }
            10 => { self.m_infection_variant.as_ref().unwrap().encode(bitstream); }
            _ => {
                panic!("Tried to encode an unsupported game engine! ({})", self.m_game_engine_index);
            }
        }

    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        self.m_game_engine_index = bitstream.read_integer(4);
        self.m_base_variant.decode(bitstream);

        match self.m_game_engine_index {
            0 => { }
            1 => {
                self.m_ctf_variant = Some(c_game_engine_ctf_variant::default());
                self.m_ctf_variant.as_ref().unwrap().decode(bitstream);
            }
            2 => {
                self.m_slayer_variant = Some(c_game_engine_slayer_variant::default());
                self.m_slayer_variant.as_ref().unwrap().decode(bitstream);
            }
            3 => {
                self.m_oddball_variant = Some(c_game_engine_oddball_variant::default());
                self.m_oddball_variant.as_ref().unwrap().decode(bitstream);
            }
            4 => {
                self.m_king_variant = Some(c_game_engine_king_variant::default());
                self.m_king_variant.as_ref().unwrap().decode(bitstream);
            }
            5 => {
                self.m_sandbox_variant = Some(c_game_engine_sandbox_variant::default());
                self.m_sandbox_variant.as_ref().unwrap().decode(bitstream);
            }
            6 => {
                self.m_vip_variant = Some(c_game_engine_vip_variant::default());
                self.m_vip_variant.as_ref().unwrap().decode(bitstream);
            }
            7 => {
                self.m_juggernaut_variant = Some(c_game_engine_juggernaut_variant::default());
                self.m_juggernaut_variant.as_ref().unwrap().decode(bitstream);
            }
            8 => {
                self.m_territories_variant = Some(c_game_engine_territories_variant::default());
                self.m_territories_variant.as_ref().unwrap().decode(bitstream);
            }
            9 => {
                self.m_assault_variant = Some(c_game_engine_assault_variant::default());
                self.m_assault_variant.as_ref().unwrap().decode(bitstream);
            }
            10 => {
                self.m_infection_variant = Some(c_game_engine_infection_variant::default());
                self.m_infection_variant.as_ref().unwrap().decode(bitstream);
            }
            _ => {
                panic!("Tried to decode an unsupported game engine! ({})", self.m_game_engine_index);
            }
        }
    }
}