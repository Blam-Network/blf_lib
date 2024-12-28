use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::{BlfChunkHooks, TitleAndBuild};
use blf_lib_derive::BlfChunk;
use crate::types::build_number_identifier::build_number_identifier;
use crate::types::c_string::StaticString;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fupd", 3.1)]
#[brw(big)]
pub struct s_blf_chunk_player_data {
    pub hopper_access: u32,
    pub bungie_user_role: u32,
    pub highest_skill: u32,
    pub hopper_directory: StaticString<32>,
}

impl BlfChunkHooks for s_blf_chunk_player_data {}

impl Default for s_blf_chunk_player_data {
    fn default() -> Self {
        s_blf_chunk_player_data {
            hopper_access: 0,
            bungie_user_role: 1,
            highest_skill: 50,
            hopper_directory: StaticString::from_string("default_hoppers").unwrap(),
        }
    }
}
