use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};

#[derive(Default, PartialEq, Debug, Clone, BinRead, BinWrite, Serialize, Deserialize)]
pub struct c_game_engine_king_variant {
    m_variant_flags: u32,
    m_score_to_win: u16,
    m_moving_hill: u8,
    m_moving_hill_order: u8,
    m_uncontested_hill_bonus: i8,
    m_kill_points: i8,
    m_inside_hill_points: i8,
    m_outside_hill_points: i8,
    #[brw(pad_after = 6)] // seems sus
    m_inside_hill_traits: c_player_traits,
    // #[serde(skip_serializing,skip_deserializing)]
    // m_pad1: StaticArray<u8, 6>,
}

impl c_game_engine_king_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0));
        bitstream.write_integer(self.m_score_to_win as u32, 10);
        bitstream.write_integer(self.m_moving_hill as u32, 4);
        bitstream.write_integer(self.m_moving_hill_order as u32, 2);
        bitstream.write_signed_integer(self.m_inside_hill_points as i32, 5);
        bitstream.write_signed_integer(self.m_outside_hill_points as i32, 5);
        bitstream.write_signed_integer(self.m_uncontested_hill_bonus as i32, 5);
        bitstream.write_signed_integer(self.m_kill_points as i32, 5);
        self.m_inside_hill_traits.encode(bitstream);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool());
        self.m_score_to_win = bitstream.read_u16(10);
        self.m_moving_hill = bitstream.read_u8(4);
        self.m_moving_hill_order = bitstream.read_u8(2);
        self.m_inside_hill_points = bitstream.read_signed_integer(5) as i8;
        self.m_outside_hill_points = bitstream.read_signed_integer(5) as i8;
        self.m_uncontested_hill_bonus = bitstream.read_signed_integer(5) as i8;
        self.m_kill_points = bitstream.read_signed_integer(5) as i8;
        self.m_inside_hill_traits.decode(bitstream);
    }
}