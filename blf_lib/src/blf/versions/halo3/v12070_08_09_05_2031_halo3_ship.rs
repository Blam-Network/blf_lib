use blf_lib_derive::ChunkFactory;
use crate::blf::chunks::halo3;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::s_blf_chunk_start_of_file;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_author::s_blf_chunk_author;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::s_blf_chunk_end_of_file;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_message_of_the_day::s_blf_chunk_message_of_the_day;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_banhammer_messages::s_blf_chunk_banhammer_messages;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_matchmaking_tips::s_blf_chunk_matchmaking_tips;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_network_configuration::s_blf_chunk_network_configuration;

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
)]
pub struct v12070_08_09_05_2031_halo3_ship {}