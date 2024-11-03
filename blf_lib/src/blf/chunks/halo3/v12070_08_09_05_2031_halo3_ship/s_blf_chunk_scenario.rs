use serde::{Deserialize, Serialize};
use crate::types::c_string::StaticWcharString;
use crate::blf_chunk;
use crate::types::array::StaticArray;
use blf_lib::blam::common::cseries::language::k_language_count;
use blf_lib::blam::halo_3::release::game::game_engine_default::k_game_engine_type_count;
use crate::types::c_string::StaticString;
use blf_lib_derive::PackedSerialize;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, PackedSerialize)]
#[PackedSerialize(1, BigEndian)]
pub struct s_blf_chunk_scenario_insertion {
    pub visible: bool,
    pub flags: u8,
    pub zone_set: u16,
    __pad4: [u8;4],
    pub names: StaticArray<StaticWcharString<32>, k_language_count>,
    pub descriptions: StaticArray<StaticWcharString<128>, k_language_count>,
}

blf_chunk!(
    #[Signature("levl")]
    #[Version(3.1)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_scenario
    {
        pub map_id: u32,
        pub map_flags: u32,
        pub names: StaticArray<StaticWcharString<32>, k_language_count>,
        pub descriptions: StaticArray<StaticWcharString<128>, k_language_count>,
        pub image_file_base: StaticString<256>,
        pub scenario_path: StaticString<256>,
        pub presence_context_id: u32,
        pub sort_order: u32,
        pub multiplayer_minimum_desired_players: u8,
        pub multiplayer_maximum_desired_players: u8,
        pub engine_maximum_teams: [u8; k_game_engine_type_count],
        pub allows_saved_films: bool,
        __pad112A: [u8; 6],
        pub insertion_points: StaticArray<s_blf_chunk_scenario_insertion, 4>,
    }
);
