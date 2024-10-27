use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_packed_map_variant;
use blf_lib::blf_chunk;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, close_bitstream_writer, create_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::SerializableBlfChunk;

blf_chunk!(
    #[Signature("gvar")]
    #[Version(10.1)]
    pub struct s_blf_chunk_packed_game_variant
    {
        pub game_variant: c_game_variant,
    }
);

impl s_blf_chunk_packed_game_variant {
    pub fn create(game_variant: c_game_variant) -> Self {
        Self {
            game_variant,
        }
    }
}

impl SerializableBlfChunk for s_blf_chunk_packed_game_variant {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
        let mut bitstream = create_bitstream_writer(0x264, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        self.game_variant.encode(&mut bitstream);

        close_bitstream_writer(&mut bitstream)
    }

    fn decode_body(&mut self, buffer: &[u8]) {
        let mut bitstream = c_bitstream_reader::new(buffer, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();
        self.game_variant.decode(&mut bitstream);
    }
}