use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use crate::types::c_string::StaticString;
use crate::types::c_string::StaticWcharString;
use serde_hex::{SerHex,StrictCap};
use blf_lib::types::time::time64_t;
use crate::types::bool::s_bool;

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

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_metadata {
    pub unique_id: u64,
    pub name: StaticWcharString<0x10>,
    pub description: StaticString<128>,
    pub author: StaticString<16>,
    pub file_type: u32,
    #[brw(align_after = 4)]
    pub author_is_xuid_online: s_bool,
    #[serde(with = "SerHex::<StrictCap>")]
    pub author_id: u64,
    pub size_in_bytes: u64,
    pub date: time64_t,
    pub length_seconds: u32,
    pub campaign_id: i32,
    pub map_id: i32,
    pub game_engine_type: u32,
    pub campaign_difficulty: i32,
    pub hopper_id: i16,
    pub game_id: u64,
}

impl s_content_item_metadata {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_qword(self.unique_id, 64);
        bitstream.write_string_wchar(&self.name.get_string(), 32);
        bitstream.write_string_utf8(&self.description.get_string(), 128);
        bitstream.write_string_utf8(&self.author.get_string(), 16);
        bitstream.write_integer(self.file_type + 1, 5);
        bitstream.write_bool(self.author_is_xuid_online);
        bitstream.write_qword(self.author_id , 64);
        bitstream.write_qword(self.size_in_bytes, 64);
        bitstream.write_qword(self.date.into(), 64);
        bitstream.write_integer(self.length_seconds, 32);
        bitstream.write_signed_integer(self.campaign_id, 32);
        bitstream.write_signed_integer(self.map_id, 32);
        bitstream.write_integer(self.game_engine_type, 4);
        bitstream.write_signed_integer(self.campaign_difficulty + 1, 3);
        bitstream.write_signed_integer(self.hopper_id as i32, 16);
        bitstream.write_qword(self.game_id, 64);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        self.unique_id = bitstream.read_qword(64);
        self.name.set_string(&bitstream.read_string_whar(32)).unwrap();
        self.description.set_string(&bitstream.read_string_utf8(128)).unwrap();
        self.author.set_string(&bitstream.read_string_utf8(16)).unwrap();
        self.file_type = bitstream.read_integer(5) - 1;
        self.author_is_xuid_online = s_bool::from(bitstream.read_bool());
        self.author_id = bitstream.read_qword(64);
        self.size_in_bytes = bitstream.read_qword(64);
        self.date = time64_t::from(bitstream.read_qword(64));
        self.length_seconds = bitstream.read_integer(32);
        self.campaign_id = bitstream.read_signed_integer(32);
        self.map_id = bitstream.read_signed_integer(32);
        self.game_engine_type = bitstream.read_integer(4);
        self.campaign_difficulty = bitstream.read_signed_integer(3) - 1;
        self.hopper_id = bitstream.read_i16(16);
        self.game_id = bitstream.read_qword(64);
    }
}