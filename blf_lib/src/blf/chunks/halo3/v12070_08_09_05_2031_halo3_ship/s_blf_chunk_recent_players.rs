use std::u32;
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

// const k_recent_players_max_count: usize = -1;
#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("furp", 2.1)]
#[brw(big)]
pub struct s_blf_chunk_recent_players
{
    #[bw(try_calc(u32::try_from(players.len())))]
    player_count: u32,
    #[br(count = player_count)]
    pub players: Vec<s_blf_chunk_recent_players_player> // UTF bytes,
}

#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite)]
pub struct s_blf_chunk_recent_players_player {
    // We had this down as a u16 in the old blf tool.
    unknown1: u8,
    unknown2: u8,
    xuid: u64,
}

impl BlfChunkHooks for s_blf_chunk_recent_players {
    fn before_write(&mut self, _previously_written: &Vec<u8>) {
        // Check user count?
    }
}

impl s_blf_chunk_recent_players {
    pub fn create() -> Self {
        Self {
            players: Vec::new()
        }
    }
}