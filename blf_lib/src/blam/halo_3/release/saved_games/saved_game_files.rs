use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use crate::types::byte_limited_utf8_string::ByteLimitedUTF8String;
use crate::types::byte_limited_wchar_string::ByteLimitedWcharString;

pub const e_saved_game_file_type_none: u32 = 0xFFFFFFFF;
pub const e_saved_game_file_type_personal: u32 = 0;
pub const e_saved_game_file_type_ctf: u32 = 1;
pub const e_saved_game_file_type_slayer: u32 = 1;
pub const e_saved_game_file_type_oddball: u32 = 2;
pub const e_saved_game_file_type_king: u32 = 3;
pub const e_saved_game_file_type_juggernaut: u32 = 4;
pub const e_saved_game_file_type_territories: u32 = 5;
pub const e_saved_game_file_type_assault: u32 = 6;
pub const e_saved_game_file_type_infection: u32 = 7;
pub const e_saved_game_file_type_vip: u32 = 8;
pub const e_saved_game_file_type_usermap: u32 = 9;
pub const e_saved_game_file_type_film: u32 = 10;
pub const e_saved_game_file_type_clip: u32 = 11;
pub const e_saved_game_file_type_screenshot: u32 = 12;
pub const k_saved_game_file_type_count: u32 = 13;

#[derive(PackedSerialize, Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[PackedSerialize(4, BigEndian)]
pub struct s_content_item_metadata {
    unique_id: u64,
    name: ByteLimitedWcharString<0x10>,
    description: ByteLimitedUTF8String<128>,
    author: ByteLimitedUTF8String<16>,
    file_type: u32,
    author_is_xuid_online: bool, // padded by 3 bytes, must be pack4
    author_id: u64,
    size_in_bytes: u64,
    date: u64, // time_t probs
    length_seconds: u32,
    campaign_id: i32,
    map_id: u32,
    game_engine_type: u32,
    campaign_difficulty: i32,
    hopper_id: i8,
    game_id: u64,
}

impl s_content_item_metadata {
    // pub fn encode(&self) -> Vec<u8> {
    //
    // }
}