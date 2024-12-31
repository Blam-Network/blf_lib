use binrw::binrw;
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::BlfChunk;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("phbr", 2.1)]
#[Size(0x93)]
#[brw(big)]
pub struct s_blf_chunk_player_heartbeat_response {
    // TODO: Map
    pub unknown1: StaticArray<u8, 0x93>,
}

impl BlfChunkHooks for s_blf_chunk_player_heartbeat_response {}

