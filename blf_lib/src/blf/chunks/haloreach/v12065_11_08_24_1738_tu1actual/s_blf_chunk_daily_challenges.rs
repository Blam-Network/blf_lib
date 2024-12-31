use binrw::{binrw, BinRead, BinWrite};
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::BlfChunk;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("dcha", 3.1)]
#[brw(big)]
pub struct s_blf_chunk_daily_challenges {
    pub unknown1: StaticArray<u8, 24>,
    #[bw(try_calc(u8::try_from(daily_challenges.len())))]
    daily_challenge_count: u8,
    pub unknown2: u8,
    #[br(count = daily_challenge_count)]
    pub daily_challenges: Vec<s_blf_chunk_daily_challenges_challenge>,
}

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize,Default,BinRead,BinWrite)]
#[brw(big)]
pub struct s_blf_chunk_daily_challenges_challenge {
    pub category: u8,
    pub index: u8,
    pub reward_credits: u16,
    // TODO: Map
    pub unknown1: StaticArray<u8, 24>,
}

impl BlfChunkHooks for s_blf_chunk_daily_challenges {}

