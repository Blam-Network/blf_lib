use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_packed_map_variant;
use blf_lib::blf_chunk;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, create_bitstream_reader, create_bitstream_writer, e_bitstream_byte_order};
use blf_lib::types::array::Array;
use blf_lib::types::byte_limited_utf8_string::FixedSizeUTF8String;
use blf_lib_derivable::blf::chunks::SerializableBlfChunk;
use crate::io::bitstream::close_bitstream_writer;

#[derive(Clone, Default, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub struct s_blf_chunk_game_set_entry {
    pub weight: u32,
    pub minimum_player_count: u8,
    pub skip_after_veto: bool,
    pub optional: bool,
    pub map_id: u32,
    pub map_variant_file_hash: s_network_http_request_hash,
    pub map_variant_file_name: FixedSizeUTF8String<32>,
    pub game_variant_file_hash: s_network_http_request_hash,
    pub game_variant_file_name: FixedSizeUTF8String<32>,
}

pub const k_maximum_game_sets: usize = 63;

blf_chunk!(
    #[Signature("gset")]
    #[Version(6.1)]
    pub struct s_blf_chunk_game_set
    {
        pub game_entry_count: usize,
        pub game_entries: Vec<s_blf_chunk_game_set_entry>,
    }
);

impl SerializableBlfChunk for s_blf_chunk_game_set {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
        let mut bitstream = create_bitstream_writer(0x1BC0, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        bitstream.write_integer(self.game_entry_count as u32, 6);

        for i in 0..self.game_entry_count {
            let game_entry = self.game_entries[i];
            bitstream.write_integer(game_entry.weight, 32);
            bitstream.write_integer(game_entry.minimum_player_count as u32, 4);
            bitstream.write_bool(game_entry.skip_after_veto);
            bitstream.write_bool(game_entry.optional);
            bitstream.write_integer(game_entry.map_id, 32);
            bitstream.write_string_utf8(&game_entry.game_variant_file_name.get_string(), 32);
            bitstream.write_raw_data(&game_entry.game_variant_file_hash.data, 0x100);
            bitstream.write_string_utf8(&game_entry.map_variant_file_name.get_string(), 32);
            bitstream.write_raw_data(&game_entry.map_variant_file_hash.data, 0x100);
        }

        close_bitstream_writer(&mut bitstream)
    }

    fn decode_body(&mut self, buffer: &[u8]) {
        let mut bitstream = create_bitstream_reader(buffer, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        self.game_entry_count = bitstream.read_integer(6) as usize;
        self.game_entries.resize(self.game_entry_count, s_blf_chunk_game_set_entry::default());

        for i in 0..self.game_entry_count {
            let game_entry = &mut self.game_entries.as_mut_slice()[i];
            game_entry.weight = bitstream.read_integer(32);
            game_entry.minimum_player_count = bitstream.read_integer(4) as u8;
            game_entry.skip_after_veto = bitstream.read_bool();
            game_entry.optional = bitstream.read_bool();
            game_entry.map_id = bitstream.read_integer(32);
            game_entry.game_variant_file_name.set_string(&bitstream.read_string_utf8(32)).unwrap();
            game_entry.game_variant_file_hash = s_network_http_request_hash::try_from(bitstream.read_raw_data(0xA0)).unwrap();
            game_entry.map_variant_file_name.set_string(&bitstream.read_string_utf8(32)).unwrap();
            game_entry.map_variant_file_hash = s_network_http_request_hash::try_from(bitstream.read_raw_data(0xA0)).unwrap();
        }
    }
}