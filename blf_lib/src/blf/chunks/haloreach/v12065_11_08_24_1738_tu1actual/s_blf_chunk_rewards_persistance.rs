use binrw::binrw;
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::BlfChunk;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("rpdl", 2.1)]
#[Size(0x21B)]
#[brw(big)]
pub struct s_blf_chunk_rewards_persistance {
    // TODO: Map
    pub unknown1: u32,
    pub unknown2: StaticArray<u8, 0x20F>,
    pub unknown3: u32,
    pub unknown4: u32,
}

impl BlfChunkHooks for s_blf_chunk_rewards_persistance {}

