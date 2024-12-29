use std::u32;
use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("scnd", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_screenshot_data
{
    #[bw(try_calc(u32::try_from(jpeg_data.len())))]
    length: u32,
    #[br(count = length)]
    pub jpeg_data: Vec<u8>
}

impl BlfChunkHooks for s_blf_chunk_screenshot_data {}
