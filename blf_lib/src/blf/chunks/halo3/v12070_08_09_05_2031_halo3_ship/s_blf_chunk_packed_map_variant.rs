use crate::io::bitstream::e_bitstream_byte_order;
use crate::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf_chunk;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::blf::chunks::SerializableBlfChunk;

blf_chunk!(
    #[Signature("mvar")]
    #[Version(12.1)]
    pub struct s_blf_chunk_packed_map_variant
    {
        // Pads here might be aligning the map to 8
        #[serde(skip_serializing,skip_deserializing)]
        pad1: u32,
        pub map_variant: c_map_variant,
        #[serde(skip_serializing,skip_deserializing)]
        pad2: u32,
    }
);

impl s_blf_chunk_packed_map_variant {
    pub fn create(map_variant: c_map_variant) -> Self {
        Self {
            pad1: 0,
            map_variant,
            pad2: 0
        }
    }
}

impl SerializableBlfChunk for s_blf_chunk_packed_map_variant {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
        let mut data = [0u8; 0xE0A0];
        let mut bitstream = c_bitstream_writer::new(&mut data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_writing(1);
        self.map_variant.encode(&mut bitstream);
        let mut bits_remaining: usize = 0;
        bitstream.finish_writing(&mut bits_remaining);
        let mut data_length: usize = 0;
        let data = bitstream.get_data(&mut data_length);
        data[0..data_length].to_vec()
    }

    fn decode_body(&mut self, buffer: &[u8]) {
        let mut bitstream = c_bitstream_reader::new(buffer, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();
        self.map_variant.decode(&mut bitstream);
    }
}