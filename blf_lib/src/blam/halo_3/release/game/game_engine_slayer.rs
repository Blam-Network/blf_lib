use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib_derive::PackedSerialize;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_slayer_variant {
    m_score_to_win: u16,
    m_kill_points: u16,
    m_assist_points: u8,
    m_death_points: u8,
    m_suicide_points: u8,
    m_betrayal_points: u8,
    m_leader_killed_points: u8,
    m_elimination_points: u8,
    m_assassination_points: u8,
    m_headshot_points: u8,
    m_melee_points: u8,
    m_sticky_points: u8,
    m_splatter_points: u8,
    m_killing_spree_points: u8,
    m_leader_traits: c_player_traits,
}

impl c_game_engine_slayer_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_integer(self.m_score_to_win as u32, 10);
        bitstream.write_integer(self.m_kill_points as u32, 5);
        bitstream.write_integer(self.m_assist_points as u32, 5);
        bitstream.write_integer(self.m_death_points as u32, 5);
        bitstream.write_integer(self.m_suicide_points as u32, 5);
        bitstream.write_integer(self.m_betrayal_points as u32, 5);
        bitstream.write_integer(self.m_leader_killed_points as u32, 5);
        bitstream.write_integer(self.m_elimination_points as u32, 5);
        bitstream.write_integer(self.m_assassination_points as u32, 5);
        bitstream.write_integer(self.m_headshot_points as u32, 5);
        bitstream.write_integer(self.m_melee_points as u32, 5);
        bitstream.write_integer(self.m_sticky_points as u32, 5);
        bitstream.write_integer(self.m_splatter_points as u32, 5);
        bitstream.write_integer(self.m_killing_spree_points as u32, 5);
        self.m_leader_traits.encode(bitstream);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        self.m_score_to_win = bitstream.read_u16(10);
        self.m_kill_points = bitstream.read_u16(5);
        self.m_assist_points = bitstream.read_u8(5);
        self.m_death_points = bitstream.read_u8(5);
        self.m_suicide_points = bitstream.read_u8(5);
        self.m_betrayal_points = bitstream.read_u8(5);
        self.m_leader_killed_points = bitstream.read_u8(5);
        self.m_elimination_points = bitstream.read_u8(5);
        self.m_assassination_points = bitstream.read_u8(5);
        self.m_headshot_points = bitstream.read_u8(5);
        self.m_melee_points = bitstream.read_u8(5);
        self.m_sticky_points = bitstream.read_u8(5);
        self.m_splatter_points = bitstream.read_u8(5);
        self.m_killing_spree_points = bitstream.read_u8(5);
        self.m_leader_traits.decode(bitstream);
    }
}