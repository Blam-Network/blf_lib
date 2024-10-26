use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_ctf_variant {
    m_variant_flags: u8,
    m_home_flag_waypoint: u8,
    m_game_type: u8,
    m_respawn: u8,
    m_touch_return_timeout: u16,
    m_sudden_death_time: u16,
    m_score_to_win: u16,
    m_flag_reset_time: u16,
    m_carrier_traits: c_player_traits,
}

impl c_game_engine_ctf_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0));
        bitstream.write_integer(self.m_home_flag_waypoint as u32, 2);
        bitstream.write_integer(self.m_game_type as u32, 2);
        bitstream.write_integer(self.m_respawn as u32, 2);
        bitstream.write_integer(self.m_score_to_win as u32, 6);
        bitstream.write_integer(self.m_sudden_death_time as u32, 9);
        bitstream.write_integer(self.m_flag_reset_time as u32, 9);
        bitstream.write_integer(self.m_touch_return_timeout as u32, 9);
        self.m_carrier_traits.encode(bitstream);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool());
        self.m_home_flag_waypoint = bitstream.read_u8(2);
        self.m_game_type = bitstream.read_u8(2);
        self.m_respawn = bitstream.read_u8(2);
        self.m_score_to_win = bitstream.read_u16(6);
        self.m_sudden_death_time = bitstream.read_u16(9);
        self.m_flag_reset_time = bitstream.read_u16(9);
        self.m_touch_return_timeout = bitstream.read_u16(9);
        self.m_carrier_traits.decode(bitstream);
    }
}