use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_sandbox_variant {
    m_variant_flags: u8,
    m_edit_mode: u8,
    m_respawn_time: u16,
    m_player_traits: c_player_traits,
}

impl c_game_engine_sandbox_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0));
        bitstream.write_integer(self.m_edit_mode as u32, 2);
        bitstream.write_integer(self.m_respawn_time as u32, 6);
        self.m_player_traits.encode(bitstream);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool());
        self.m_edit_mode = bitstream.read_u8(2);
        self.m_respawn_time = bitstream.read_u16(6);
        self.m_player_traits.decode(bitstream);
    }
}