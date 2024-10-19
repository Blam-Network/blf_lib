use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::bitstream::c_bitstream;
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
    pub fn encode(&self, bitstream: &mut c_bitstream) {
        bitstream.write_qword(self.unique_id, 64);
        bitstream.write_string_wchar(&self.name.get_string(), 32);
        bitstream.write_string_utf8(&self.description.get_string(), 128);
        bitstream.write_string_utf8(&self.author.get_string(), 16);
        bitstream.write_integer(self.file_type, 5);
        bitstream.write_integer(if self.author_is_xuid_online { 1 } else { 0 }, 1);
        bitstream.write_qword(self.author_id , 64);
        bitstream.write_qword(self.size_in_bytes, 64);
        bitstream.write_qword(self.date, 64);
        bitstream.write_integer(self.length_seconds, 32);
        bitstream.write_integer(self.campaign_id as u32, 32);
        bitstream.write_integer(self.map_id, 32);
        bitstream.write_integer(self.game_engine_type, 4);
        bitstream.write_integer(self.campaign_difficulty as u32, 3);
        bitstream.write_integer(self.hopper_id as u32, 16);
        bitstream.write_qword(self.game_id, 64);
    }
}