use crate::io::bitstream::{create_bitstream_writer, e_bitstream_byte_order};
use crate::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf_chunk;
use blf_lib::io::bitstream::{c_bitstream_reader, close_bitstream_writer};
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
        let mut bitstream = create_bitstream_writer(0xE0A0, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        self.map_variant.encode(&mut bitstream);

        close_bitstream_writer(&mut bitstream)
    }

    fn decode_body(&mut self, buffer: &[u8]) {
        let mut bitstream = c_bitstream_reader::new(buffer, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();
        self.map_variant.decode(&mut bitstream);
    }
}