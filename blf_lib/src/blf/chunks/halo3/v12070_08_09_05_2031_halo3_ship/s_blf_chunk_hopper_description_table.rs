use serde::{Deserialize, Serialize};
use blf_lib::blf_chunk;
use blf_lib::io::bitstream::{create_bitstream_reader, create_bitstream_writer, e_bitstream_byte_order};
use blf_lib::types::byte_limited_utf8_string::StaticString;
use blf_lib_derivable::blf::chunks::SerializableBlfChunk;
use crate::io::bitstream::close_bitstream_writer;

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct s_game_hopper_description {
    pub identifier: u16,
    pub hopper_type: bool,
    pub description: StaticString<256>,
}

pub const MAX_DESCRIPTIONS: usize = 63;

blf_chunk!(
    #[Signature("mhdf")]
    #[Version(3.1)]
    pub struct s_blf_chunk_hopper_description_table {
        pub description_count: usize,
        pub descriptions: Vec<s_game_hopper_description>,
    }
);

impl SerializableBlfChunk for s_blf_chunk_hopper_description_table {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
        let mut bitstream = create_bitstream_writer(0x4204, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        bitstream.write_integer(self.description_count as u32, 6);

        for i in 0..self.description_count {
            let description = &self.descriptions[i];
            bitstream.write_integer(description.identifier as u32, 16);
            bitstream.write_bool(description.hopper_type);
            bitstream.write_string_utf8(&description.description.get_string(), 256);
        }

        close_bitstream_writer(&mut bitstream)
    }

    fn decode_body(&mut self, buffer: &[u8]) {
        let mut bitstream = create_bitstream_reader(buffer, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        self.description_count = bitstream.read_integer(6) as usize;
        self.descriptions.resize(self.description_count, s_game_hopper_description::default());

        for i in 0..self.description_count {
            let description = &mut self.descriptions[i];
            description.identifier = bitstream.read_integer(16) as u16;
            description.hopper_type = bitstream.read_bool();
            description.description.set_string(&bitstream.read_string_utf8(256)).unwrap();
        }
    }
}