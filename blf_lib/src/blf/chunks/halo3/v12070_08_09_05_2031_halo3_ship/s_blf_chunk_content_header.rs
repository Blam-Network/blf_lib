use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blam::halo_3::release::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("chdr", 9.2)]
#[brw(big)]
pub struct s_blf_chunk_content_header
{
    pub build_number: u16,
    pub map_minor_version: u16,
    pub metadata: s_content_item_metadata,
}

impl BlfChunkHooks for s_blf_chunk_content_header {}

impl s_blf_chunk_content_header {
    pub fn create_for_game_variant(game_variant: &c_game_variant) -> s_blf_chunk_content_header {
        s_blf_chunk_content_header { 
            build_number: 12070,
            map_minor_version: 0,
            metadata: game_variant.m_base_variant.m_metadata.clone(),
        }
    }

    pub fn create_for_map_variant(map_variant: &c_map_variant) -> s_blf_chunk_content_header {
        s_blf_chunk_content_header {
            build_number: 12070,
            map_minor_version: 0,
            metadata: map_variant.m_metadata.clone(),
        }
    }
}