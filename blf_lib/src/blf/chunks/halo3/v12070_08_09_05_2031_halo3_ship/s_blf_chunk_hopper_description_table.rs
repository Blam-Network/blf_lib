use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, create_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;
use crate::io::bitstream::close_bitstream_writer;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Signature("mhdf")]
#[Version(3.1)]
pub struct s_blf_chunk_hopper_description_table {
    description_count: usize,
    descriptions: Vec<s_game_hopper_description>,
}

impl BlfChunkHooks for s_blf_chunk_hopper_description_table {}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct s_game_hopper_description {
    pub identifier: u16,
    hopper_type: bool, // restricted?
    pub description: StaticString<256>,
}


pub const MAX_DESCRIPTIONS: usize = 63;

impl s_blf_chunk_hopper_description_table {
    pub fn get_descriptions(&self) -> Vec<s_game_hopper_description> {
        self.descriptions.as_slice()[0..self.description_count].to_vec()
    }

    pub fn add_description(&mut self, config: (u16, &String)) -> Result<(), String> {
        if self.description_count >= MAX_DESCRIPTIONS {
            return Err("The hopper desciptions chunk is full!".to_string());
        }
        self.description_count += 1;
        self.descriptions.push(s_game_hopper_description {
            identifier: config.0,
            hopper_type: false, // seems unused
            description: StaticString::from_string(config.1)?,
        });
        Ok(())
    }
}


impl BinRead for s_blf_chunk_hopper_description_table {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)?;

        let mut bitstream = c_bitstream_reader::new(buffer.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();

        let mut mhdf = Self::default();

        mhdf.description_count = bitstream.read_integer(6) as usize;
        mhdf.descriptions.resize(mhdf.description_count, s_game_hopper_description::default());

        for i in 0..mhdf.description_count {
            let description = &mut mhdf.descriptions[i];
            description.identifier = bitstream.read_integer(16) as u16;
            description.hopper_type = bitstream.read_bool();
            description.description.set_string(&bitstream.read_string_utf8(256)).unwrap();
        }

        Ok(mhdf)
    }
}

impl BinWrite for s_blf_chunk_hopper_description_table {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let mut bitstream = create_bitstream_writer(0x4000, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        bitstream.write_integer(self.description_count as u32, 6);

        for i in 0..self.description_count {
            let description = &self.descriptions[i];
            bitstream.write_integer(description.identifier as u32, 16);
            bitstream.write_bool(description.hopper_type);
            bitstream.write_string_utf8(&description.description.get_string(), 256);
        }

        writer.write_ne(&close_bitstream_writer(&mut bitstream))
    }
}
