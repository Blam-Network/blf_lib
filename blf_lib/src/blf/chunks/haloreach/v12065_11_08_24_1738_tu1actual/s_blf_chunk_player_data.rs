use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};
use crate::types::array::StaticArray;
use crate::types::c_string::StaticString;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fupd", 7.1)]
#[brw(big)]
#[Size(0x45)]
pub struct s_blf_chunk_player_data {
    pub hopper_access: u8,
    pub bungie_user_role: u16,
    pub unknown1: u8,
    pub hopper_directory: StaticString<32>,
    pub unknown2: StaticArray<u8, 0x20>,
    pub unknown3: u8,
}

impl BlfChunkHooks for s_blf_chunk_player_data {}

impl Default for s_blf_chunk_player_data {
    fn default() -> Self {
        s_blf_chunk_player_data {
            hopper_access: 0,
            bungie_user_role: 0xffff,
            unknown1: 0,
            hopper_directory: StaticString::from_string("default_hoppers").unwrap(),
            unknown2: StaticArray::default(),
            unknown3: 1,
        }
    }
}
