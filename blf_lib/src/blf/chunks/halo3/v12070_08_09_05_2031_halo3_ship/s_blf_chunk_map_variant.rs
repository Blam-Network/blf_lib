use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use crate::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mapv", 12.1)]
#[brw(big)]
pub struct s_blf_chunk_map_variant
{
    // Pads here might be aligning the map to 8
    // #[serde(skip_serializing,skip_deserializing)]
    // pad1: u32,
    #[brw(pad_before = 4, pad_after = 4)]
    pub map_variant: c_map_variant,
    // #[serde(skip_serializing,skip_deserializing)]
    // pad2: u32,
}

impl BlfChunkHooks for s_blf_chunk_map_variant {}

impl s_blf_chunk_map_variant {
    pub fn create(map_variant: c_map_variant) -> Self {
        Self {
            // pad1: 0,
            map_variant,
            // pad2: 0,
        }
    }
}