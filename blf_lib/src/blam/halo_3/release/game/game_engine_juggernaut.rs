use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use crate::types::array::StaticArray;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_juggernaut_variant {
    m_score_to_win_round: u16,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad1: StaticArray<u8, 2>,
    m_initial_juggernaut: u8,
    m_next_juggernaut: u8,
    m_variant_flags: u8,
    m_zone_movement: u8,
    m_zone_order: u8,
    m_kill_points: i8,
    m_juggernaut_kill_points: i8,
    m_kill_as_juggernaut_points: i8,
    m_destination_arrival_points: i8,
    m_suicide_points: i8,
    m_betrayal_points: i8,
    m_juggernaut_delay: u8,
    m_juggernaut_traits: c_player_traits,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad2: StaticArray<u8, 4>,
}

impl c_game_engine_juggernaut_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0));
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 1));
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 2));
        bitstream.write_integer(self.m_score_to_win_round as u32, 9);
        bitstream.write_integer(self.m_initial_juggernaut as u32, 2);
        bitstream.write_integer(self.m_next_juggernaut as u32, 2);
        bitstream.write_integer(self.m_zone_movement as u32, 4);
        bitstream.write_integer(self.m_zone_order as u32, 1);
        bitstream.write_signed_integer(self.m_kill_points as i32, 5);
        bitstream.write_signed_integer(self.m_juggernaut_kill_points as i32, 5);
        bitstream.write_signed_integer(self.m_kill_as_juggernaut_points as i32, 5);
        bitstream.write_signed_integer(self.m_destination_arrival_points as i32, 5);
        bitstream.write_signed_integer(self.m_suicide_points as i32, 5);
        bitstream.write_signed_integer(self.m_betrayal_points as i32, 5);
        bitstream.write_integer(self.m_juggernaut_delay as u32, 4);
        self.m_juggernaut_traits.encode(bitstream);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool());
        SET_BIT!(self.m_variant_flags, 1, bitstream.read_bool());
        SET_BIT!(self.m_variant_flags, 2, bitstream.read_bool());
        self.m_score_to_win_round = bitstream.read_u16(9);
        self.m_initial_juggernaut = bitstream.read_u8(2);
        self.m_next_juggernaut = bitstream.read_u8(2);
        self.m_zone_movement = bitstream.read_u8(4);
        self.m_zone_order = bitstream.read_u8(1);
        self.m_kill_points = bitstream.read_signed_integer(5) as i8;
        self.m_juggernaut_kill_points = bitstream.read_signed_integer(5) as i8;
        self.m_kill_as_juggernaut_points = bitstream.read_signed_integer(5) as i8;
        self.m_destination_arrival_points = bitstream.read_signed_integer(5) as i8;
        self.m_suicide_points = bitstream.read_signed_integer(5) as i8;
        self.m_betrayal_points = bitstream.read_signed_integer(5) as i8;
        self.m_juggernaut_delay = bitstream.read_u8(4);
        self.m_juggernaut_traits.decode(bitstream);
    }
}