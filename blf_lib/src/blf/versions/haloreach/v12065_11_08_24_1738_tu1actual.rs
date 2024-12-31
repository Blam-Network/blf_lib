use blf_lib_derive::ChunkFactory;
use crate::blf::chunks::halo3;
use crate::blf::chunks::haloreach;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_author::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_player_data::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_arena_hopper_stats::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_player_heartbeat_response::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_daily_challenges::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_rewards_persistance::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_service_record::*;


#[derive(ChunkFactory)]
#[Title("Halo: Reach")]
#[Build("12065.11.08.24.1738.tu1actual")]
#[Chunks(
    s_blf_chunk_start_of_file,
    s_blf_chunk_author,
    s_blf_chunk_end_of_file,
    s_blf_chunk_player_data,
    s_blf_chunk_arena_hopper_stats,
    s_blf_chunk_player_heartbeat_response,
    s_blf_chunk_daily_challenges,
    s_blf_chunk_rewards_persistance,
    s_blf_chunk_service_record
)]
pub struct v12065_11_08_24_1738_tu1actual {}