use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use crate::io::bitstream::{create_bitstream_writer, e_bitstream_byte_order};
use crate::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::io::bitstream::{c_bitstream_reader, close_bitstream_writer};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mvar", 12.1)]
pub struct s_blf_chunk_packed_map_variant
{
    // Pads here might be aligning the map to 8
    #[serde(skip_serializing,skip_deserializing)]
    pad1: u32,
    pub map_variant: c_map_variant,
    #[serde(skip_serializing,skip_deserializing)]
    pad2: u32,
}

impl BlfChunkHooks for s_blf_chunk_packed_map_variant {}

impl s_blf_chunk_packed_map_variant {
    pub fn create(map_variant: c_map_variant) -> Self {
        Self {
            pad1: 0,
            map_variant,
            pad2: 0
        }
    }
}

impl BinRead for s_blf_chunk_packed_map_variant {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)?;

        let mut bitstream = c_bitstream_reader::new(buffer.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();

        let mut packed_map_variant = Self::default();

        packed_map_variant.map_variant.decode(&mut bitstream);

        Ok(packed_map_variant)
    }
}

impl BinWrite for s_blf_chunk_packed_map_variant {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let mut bitstream = create_bitstream_writer(0xE0A0, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        self.map_variant.encode(&mut bitstream);
        writer.write_ne(&close_bitstream_writer(&mut bitstream))
    }
}
