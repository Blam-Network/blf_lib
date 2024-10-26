use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blf_chunk;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::SerializableBlfChunk;

blf_chunk!(
    #[Signature("gvar")]
    #[Version(10.1)]
    pub struct s_blf_chunk_packed_game_variant
    {
        pub game_variant: c_game_variant,
    }
);

impl SerializableBlfChunk for s_blf_chunk_packed_game_variant {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
        let mut data = [0u8; 0x264]; // more than we need.
        let mut bitstream = c_bitstream_writer::new(&mut data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_writing(1);
        self.game_variant.encode(&mut bitstream);
        let mut bits_remaining: usize = 0;
        bitstream.finish_writing(&mut bits_remaining);
        let mut data_length: usize = 0;
        let data = bitstream.get_data(&mut data_length);
        data[0..data_length].to_vec()
    }

    fn decode_body(&mut self, buffer: &[u8]) {
        let mut bitstream = c_bitstream_reader::new(buffer, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();
        self.game_variant.decode(&mut bitstream);
    }
}