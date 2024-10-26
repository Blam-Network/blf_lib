use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_default::c_game_engine_base_variant;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib_derive::PackedSerialize;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_miscellaneous_options {
    m_flags: u8,
    m_round_time_limit_minutes: u8,
    m_round_limit: u8,
    m_early_victory_win_count: u8,
}

impl c_game_engine_miscellaneous_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0));
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1));
        bitstream.write_bool(TEST_BIT!(self.m_flags, 2));
        bitstream.write_integer(self.m_round_time_limit_minutes as u32, 8);
        bitstream.write_integer(self.m_round_limit as u32, 4);
        bitstream.write_integer(self.m_early_victory_win_count as u32, 4);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_flags, 0, bitstream.read_bool());
        SET_BIT!(self.m_flags, 1, bitstream.read_bool());
        SET_BIT!(self.m_flags, 2, bitstream.read_bool());
        self.m_round_time_limit_minutes = bitstream.read_u8(8);
        self.m_round_limit = bitstream.read_u8(4);
        self.m_early_victory_win_count = bitstream.read_u8(4);
    }
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_respawn_options {
    m_flags: u8,
    m_lives_per_round: u8,
    m_team_lives_per_round: u8,
    m_respawn_time_seconds: u8,
    m_suicide_penalty_seconds: u8,
    m_betrayal_penalty_seconds: u8,
    // m_unknown_penalty_seconds: u8,
    m_respawn_growth_seconds: u8,
    m_respawn_player_traits_duration_seconds: u8,
    m_respawn_player_traits: c_player_traits,
}

impl c_game_engine_respawn_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0));
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1));
        bitstream.write_bool(TEST_BIT!(self.m_flags, 2));
        bitstream.write_bool(TEST_BIT!(self.m_flags, 3));
        bitstream.write_integer(self.m_lives_per_round as u32, 6);
        bitstream.write_integer(self.m_team_lives_per_round as u32, 7);
        bitstream.write_integer(self.m_respawn_time_seconds as u32, 8);
        bitstream.write_integer(self.m_suicide_penalty_seconds as u32, 8);
        bitstream.write_integer(self.m_betrayal_penalty_seconds as u32, 8);
        bitstream.write_integer(self.m_respawn_growth_seconds as u32, 4);
        bitstream.write_integer(self.m_respawn_player_traits_duration_seconds as u32, 6);
        self.m_respawn_player_traits.encode(bitstream);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_flags, 0, bitstream.read_bool());
        SET_BIT!(self.m_flags, 1, bitstream.read_bool());
        SET_BIT!(self.m_flags, 2, bitstream.read_bool());
        SET_BIT!(self.m_flags, 3, bitstream.read_bool());
        self.m_lives_per_round = bitstream.read_u8(6);
        self.m_team_lives_per_round = bitstream.read_u8(7);
        self.m_respawn_time_seconds = bitstream.read_u8(8);
        self.m_suicide_penalty_seconds = bitstream.read_u8(8);
        self.m_betrayal_penalty_seconds = bitstream.read_u8(8);
        self.m_respawn_growth_seconds = bitstream.read_u8(4);
        self.m_respawn_player_traits_duration_seconds = bitstream.read_u8(6);
        self.m_respawn_player_traits.decode(bitstream);
    }
}