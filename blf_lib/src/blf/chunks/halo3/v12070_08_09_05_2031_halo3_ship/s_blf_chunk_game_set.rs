use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::io::bitstream::{c_bitstream_reader, create_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;
use crate::io::bitstream::close_bitstream_writer;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Signature("gset")]
#[Version(6.1)]
pub struct s_blf_chunk_game_set
{
    game_entry_count: usize,
    game_entries: Vec<s_blf_chunk_game_set_entry>,
}

impl BlfChunkHooks for s_blf_chunk_game_set {}

#[derive(Clone, Default, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub struct s_blf_chunk_game_set_entry {
    pub weight: u32,
    pub minimum_player_count: u8,
    pub skip_after_veto: bool,
    pub optional: bool,
    pub map_id: u32,
    pub map_variant_file_hash: s_network_http_request_hash,
    pub map_variant_file_name: StaticString<32>,
    pub game_variant_file_hash: s_network_http_request_hash,
    pub game_variant_file_name: StaticString<32>,
}

pub const k_maximum_game_sets: usize = 63;

impl s_blf_chunk_game_set {
    pub fn get_entries(&self) -> Vec<s_blf_chunk_game_set_entry> {
        self.game_entries.clone()
    }

    pub fn add_entry(&mut self, entry: s_blf_chunk_game_set_entry) -> Result<(),String> {
        if self.game_entry_count == k_maximum_game_sets {
            return Err("Tried to add an entry to a full game set!".to_string())
        }

        self.game_entries.push(entry);
        self.game_entry_count = self.game_entries.len();
        Ok(())
    }
}

impl BinRead for s_blf_chunk_game_set {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)?;

        let mut bitstream = c_bitstream_reader::new(buffer.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();

        let mut game_set = Self::default();

        game_set.game_entry_count = bitstream.read_integer(6) as usize;
        game_set.game_entries.resize(game_set.game_entry_count, s_blf_chunk_game_set_entry::default());

        for i in 0..game_set.game_entry_count {
            let game_entry = &mut game_set.game_entries.as_mut_slice()[i];
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

        Ok(game_set)
    }
}

impl BinWrite for s_blf_chunk_game_set {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
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
            bitstream.write_raw_data(&game_entry.game_variant_file_hash.data, 0xA0);
            bitstream.write_string_utf8(&game_entry.map_variant_file_name.get_string(), 32);
            bitstream.write_raw_data(&game_entry.map_variant_file_hash.data, 0xA0);
        }

        writer.write_ne(&close_bitstream_writer(&mut bitstream))
    }
}
