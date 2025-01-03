use std::u32;
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::types::bool::s_bool;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};
use blf_lib::types::array::StaticArray;
use blf_lib::types::c_string::StaticWcharString;
use blf_lib::types::time::time64_t;
#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("filq", 1.1)]
#[brw(big)]
#[Size(0x280)]
pub struct s_blf_chunk_file_transfers
{
    pub transfers: StaticArray<s_blf_chunk_file_transfers_transfer, 8>
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_blf_chunk_file_transfers_transfer {
    pub player_xuid: u64,
    pub slot: u32,
    pub unknown_c: u32,
    pub server_id: u64,
    pub file_name: StaticWcharString<16>,
    pub file_type: u32,
    pub unknown_3c: u32,
    pub map_id: u32,
    pub unknown_44: u32,
    pub unknown_48: u32,
    pub size_bytes: u32,
}

impl BlfChunkHooks for s_blf_chunk_file_transfers { }
