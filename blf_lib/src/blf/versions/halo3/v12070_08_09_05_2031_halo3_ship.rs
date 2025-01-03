use blf_lib_derive::ChunkFactory;
use crate::blf::chunks::halo3;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_author::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_message_of_the_day::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_banhammer_messages::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_matchmaking_tips::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_network_configuration::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_map_manifest::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_message_of_the_day_popup::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_map_variant::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_packed_map_variant::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_game_variant::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_packed_game_variant::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_game_set::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_hopper_configuration_table::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_hopper_description_table::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_scenario::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_online_file_manifest::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_content_header::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_recent_players::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_player_data::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_machine_network_statistics::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_screenshot_data::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_screenshot_camera::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_compressed_data::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_service_record::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_user_bans::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_file_transfers::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_matchmaking_hopper_statistics::*;

#[derive(ChunkFactory)]
#[Title("Halo 3")]
#[Build("12070.08.09.05.2031.halo3_ship")]
#[Chunks(
    s_blf_chunk_start_of_file, 
    s_blf_chunk_author,
    s_blf_chunk_end_of_file,
    s_blf_chunk_message_of_the_day,
    s_blf_chunk_banhammer_messages,
    s_blf_chunk_matchmaking_tips,
    s_blf_chunk_network_configuration,
    s_blf_chunk_map_manifest,
    s_blf_chunk_message_of_the_day_popup,
    s_blf_chunk_map_variant,
    s_blf_chunk_packed_map_variant,
    s_blf_chunk_game_variant,
    s_blf_chunk_packed_game_variant,
    s_blf_chunk_game_set,
    s_blf_chunk_hopper_configuration_table,
    s_blf_chunk_hopper_description_table,
    s_blf_chunk_scenario,
    s_blf_chunk_online_file_manifest,
    s_blf_chunk_content_header,
    s_blf_chunk_recent_players,
    s_blf_chunk_player_data,
    s_blf_chunk_screenshot_data,
    s_blf_chunk_screenshot_camera,
    s_blf_chunk_service_record,
    s_blf_chunk_user_bans,
    s_blf_chunk_file_transfers,
    s_blf_chunk_matchmaking_hopper_statistics,
)]
pub struct v12070_08_09_05_2031_halo3_ship {}