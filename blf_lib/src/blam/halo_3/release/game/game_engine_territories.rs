use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};

#[derive(Default, PartialEq, Debug, Clone, BinRead, BinWrite, Serialize, Deserialize)]
pub struct c_game_engine_territories_variant {
    m_variant_flags: u16,
    m_respawn_on_capture: u16,
    m_capture_time: u16,
    m_sudden_death_time: i16,
    m_defender_traits: c_player_traits,
    m_attacker_traits: c_player_traits,
}

impl c_game_engine_territories_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0));
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 1));
        bitstream.write_integer(self.m_respawn_on_capture as u32, 2);
        bitstream.write_integer(self.m_capture_time as u32, 7);
        bitstream.write_signed_integer(self.m_sudden_death_time as i32, 10);
        self.m_defender_traits.encode(bitstream);
        self.m_attacker_traits.encode(bitstream);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool());
        SET_BIT!(self.m_variant_flags, 1, bitstream.read_bool());
        self.m_respawn_on_capture = bitstream.read_u16(2);
        self.m_capture_time = bitstream.read_u16(7);
        self.m_sudden_death_time = bitstream.read_signed_integer(10) as i16;
        self.m_defender_traits.decode(bitstream);
        self.m_attacker_traits.decode(bitstream);
    }
}