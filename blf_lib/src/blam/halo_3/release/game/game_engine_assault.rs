use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use crate::types::array::StaticArray;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_assault_variant {
    m_variant_flags: u16,
    m_respawn: u16,
    m_game_type: u16,
    m_enemy_bomb_waypoint: u16,
    m_score_to_win: u16,
    m_sudden_death_time: i16,
    m_bomb_reset_time: u16,
    m_bomb_arming_time: u16,
    m_bomb_disarming_time: u16,
    m_bomb_fuse_time: u16,
    m_carrier_traits: c_player_traits,
    m_arming_traits: c_player_traits,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad1: StaticArray<u8, 4>,
}

impl c_game_engine_assault_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0));
        bitstream.write_integer(self.m_game_type as u32, 2);
        bitstream.write_integer(self.m_respawn as u32, 3);
        bitstream.write_integer(self.m_enemy_bomb_waypoint as u32, 3);
        bitstream.write_integer(self.m_score_to_win as u32, 6);
        bitstream.write_signed_integer(self.m_sudden_death_time as i32, 9);
        bitstream.write_integer(self.m_bomb_arming_time as u32, 5);
        bitstream.write_integer(self.m_bomb_disarming_time as u32, 5);
        bitstream.write_integer(self.m_bomb_fuse_time as u32, 5);
        bitstream.write_integer(self.m_bomb_reset_time as u32, 6);
        self.m_carrier_traits.encode(bitstream);
        self.m_arming_traits.encode(bitstream);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool());
        self.m_game_type = bitstream.read_integer(2) as u16;
        self.m_respawn = bitstream.read_integer(3) as u16;
        self.m_enemy_bomb_waypoint = bitstream.read_integer(3) as u16;
        self.m_score_to_win = bitstream.read_integer(6) as u16;
        self.m_sudden_death_time = bitstream.read_signed_integer(9) as i16;
        self.m_bomb_arming_time = bitstream.read_integer(5) as u16;
        self.m_bomb_disarming_time = bitstream.read_integer(5) as u16;
        self.m_bomb_fuse_time = bitstream.read_integer(5) as u16;
        self.m_bomb_reset_time = bitstream.read_integer(6) as u16;
        self.m_carrier_traits.decode(bitstream);
        self.m_arming_traits.decode(bitstream);
    }
}