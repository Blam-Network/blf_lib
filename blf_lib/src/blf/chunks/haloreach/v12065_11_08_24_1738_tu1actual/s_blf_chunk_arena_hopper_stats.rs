use binrw::binrw;
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::BlfChunk;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("arhs", 3.1)]
#[brw(big)]
pub struct s_blf_chunk_arena_hopper_stats {
    // TODO: Map
    data: StaticArray<u8, 0x16>,
}

impl BlfChunkHooks for s_blf_chunk_arena_hopper_stats {}

