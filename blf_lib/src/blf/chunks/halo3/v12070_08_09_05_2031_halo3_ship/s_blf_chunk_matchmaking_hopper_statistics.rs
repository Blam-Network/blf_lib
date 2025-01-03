use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("mmhs", 3.1)]
#[brw(big)]
#[Size(0x104)]
pub struct s_blf_chunk_matchmaking_hopper_statistics {
    pub player_count: u32,
    data: StaticArray<hopper_population, 32>,
}

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize,Default,BinRead,BinWrite)]
pub struct hopper_population {
    pub hopper_identifier: u32,
    pub player_count: u32,
}

impl BlfChunkHooks for s_blf_chunk_matchmaking_hopper_statistics {}

