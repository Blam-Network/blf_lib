use std::u32;
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mapi", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_map_image
{
    #[brw(pad_before = 3)]
    pub image_type: e_map_image_type,
    #[bw(try_calc(u32::try_from(image_data.len())))]
    length: u32,
    #[br(count = length)]
    pub image_data: Vec<u8>
}

impl BlfChunkHooks for s_blf_chunk_map_image {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u8)]
pub enum e_map_image_type {
    #[default]
    map_image_type_jpg = 0,
    map_image_type_png = 1,
}
