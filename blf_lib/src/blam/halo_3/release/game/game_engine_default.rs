use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::blam::halo_3::release::game::game_engine_traits::{c_game_engine_miscellaneous_options, c_game_engine_respawn_options};
use blf_lib::types::array::Array;
use blf_lib_derive::PackedSerialize;
use crate::blam::halo_3::release::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_social_options {
    m_flags: u16,
    m_team_changing: u16,
}

impl c_game_engine_social_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(false); // "guess i'll go fuck myself" - observers
        bitstream.write_integer(self.m_team_changing as u32, 2);
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0));
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1));
        bitstream.write_bool(TEST_BIT!(self.m_flags, 2));
        bitstream.write_bool(TEST_BIT!(self.m_flags, 3));
        bitstream.write_bool(TEST_BIT!(self.m_flags, 4));
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        bitstream.read_bool();
        self.m_team_changing = bitstream.read_integer(2) as u16;
        SET_BIT!(self.m_flags, 0, bitstream.read_bool());
        SET_BIT!(self.m_flags, 1, bitstream.read_bool());
        SET_BIT!(self.m_flags, 2, bitstream.read_bool());
        SET_BIT!(self.m_flags, 3, bitstream.read_bool());
        SET_BIT!(self.m_flags, 4, bitstream.read_bool());
    }
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_map_override_options {
    m_flags: u32,
    m_base_player_traits: c_player_traits,
    m_weapon_set_absolute_index: i16,
    m_vehicle_set_absolute_index: i16,
    m_red_powerup_traits: c_player_traits,
    m_blue_powerup_traits: c_player_traits,
    m_yellow_powerup_traits: c_player_traits,
    m_red_powerup_duration_seconds: u8,
    m_blue_powerup_duration_seconds: u8,
    m_yellow_powerup_duration_seconds: u8,
    #[serde(skip_serializing,skip_deserializing)]
    pad: u8, // john hold on
}

impl c_game_engine_map_override_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0));
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1));
        self.m_base_player_traits.encode(bitstream);
        bitstream.write_signed_integer(self.m_weapon_set_absolute_index as i32, 8);
        bitstream.write_signed_integer(self.m_vehicle_set_absolute_index as i32, 8);
        self.m_red_powerup_traits.encode(bitstream);
        self.m_blue_powerup_traits.encode(bitstream);
        self.m_yellow_powerup_traits.encode(bitstream);
        bitstream.write_integer(self.m_red_powerup_duration_seconds as u32, 7);
        bitstream.write_integer(self.m_blue_powerup_duration_seconds as u32, 7);
        bitstream.write_integer(self.m_yellow_powerup_duration_seconds as u32, 7);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_flags, 0, bitstream.read_bool());
        SET_BIT!(self.m_flags, 1, bitstream.read_bool());
        self.m_base_player_traits.decode(bitstream);
        self.m_weapon_set_absolute_index = bitstream.read_signed_integer(8) as i16;
        self.m_vehicle_set_absolute_index = bitstream.read_signed_integer(8) as i16;
        self.m_red_powerup_traits.decode(bitstream);
        self.m_blue_powerup_traits.decode(bitstream);
        self.m_yellow_powerup_traits.decode(bitstream);
        self.m_red_powerup_duration_seconds = bitstream.read_integer(7) as u8;
        self.m_blue_powerup_duration_seconds = bitstream.read_integer(7) as u8;
        self.m_yellow_powerup_duration_seconds = bitstream.read_integer(7) as u8;
    }
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_base_variant {
    #[serde(skip_serializing,skip_deserializing)]
    m_checksum: u32,
    #[serde(skip_serializing,skip_deserializing)]
    pad: Array<u32, 1>,
    m_metadata: s_content_item_metadata,
    m_miscellaneous_options: c_game_engine_miscellaneous_options,
    m_respawn_options: c_game_engine_respawn_options,
    m_social_options: c_game_engine_social_options,
    m_map_override_options: c_game_engine_map_override_options,
    pad2: u32,
    m_flags: u16,
    m_team_scoring_method: u16,
}

impl c_game_engine_base_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        self.m_metadata.encode(bitstream);
        bitstream.write_integer(self.m_flags as u32, 1);
        self.m_miscellaneous_options.encode(bitstream);
        self.m_respawn_options.encode(bitstream);
        self.m_social_options.encode(bitstream);
        self.m_map_override_options.encode(bitstream);
        bitstream.write_integer(self.m_team_scoring_method as u32, 3);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        self.m_metadata.decode(bitstream);
        self.m_flags = bitstream.read_u16(1);
        self.m_miscellaneous_options.decode(bitstream);
        self.m_respawn_options.decode(bitstream);
        self.m_social_options.decode(bitstream);
        self.m_map_override_options.decode(bitstream);
        self.m_team_scoring_method = bitstream.read_u16(3);
    }
}