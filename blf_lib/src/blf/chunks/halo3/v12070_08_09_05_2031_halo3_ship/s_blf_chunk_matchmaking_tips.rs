use std::u32;
use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;

const MAX_MATCHMAKING_TIP_COUNT: usize = 32usize;
const TIP_LENGTH: usize = 0x100;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Signature("mmtp")]
#[Version(1.1)]
#[brw(big)]
pub struct s_blf_chunk_matchmaking_tips
{
    #[bw(try_calc(u32::try_from(tips.len())))]
    tip_count: u32,
    #[br(count = tip_count)]
    pub tips: Vec<StaticString<TIP_LENGTH>> // UTF bytes,
}

impl BlfChunkHooks for s_blf_chunk_matchmaking_tips {}

impl s_blf_chunk_matchmaking_tips {
    pub fn get_tips(&self) -> Vec<String> {
        self.tips.iter().map(|tip|tip.get_string()).collect()
    }

    fn set_tips(&mut self, tips: Vec<String>) -> Result<(), String> {
        if tips.len() > MAX_MATCHMAKING_TIP_COUNT {
            return Err(format!("Too many tips! {}/{MAX_MATCHMAKING_TIP_COUNT}", tips.len()))
        }

        self.tips = Vec::with_capacity(tips.len());
        for tip in tips.iter() {
            let tip = StaticString::<TIP_LENGTH>::from_string(tip);

            if !tip.is_ok() {
                return Err(format!("Tip: {}", tip.unwrap_err()))
            }

            let tip = tip?;

            self.tips.push(tip);
        }
        // self.tip_count = self.tips.len() as u32;
        Ok(())
    }

    pub fn create(tips: Vec<String>) -> s_blf_chunk_matchmaking_tips {
        let mut new = Self::default();
        new.set_tips(tips).unwrap();
        new
    }
}