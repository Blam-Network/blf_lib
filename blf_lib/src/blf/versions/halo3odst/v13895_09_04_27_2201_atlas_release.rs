use blf_lib_derive::ChunkFactory;
use crate::blf::chunks::halo3;
use crate::blf::chunks::halo3odst;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_author::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_message_of_the_day::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_banhammer_messages::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_map_manifest::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_message_of_the_day_popup::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_scenario::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_online_file_manifest::*;
pub use halo3odst::v13895_09_04_27_2201_atlas_release::s_blf_chunk_network_configuration::*;

#[derive(ChunkFactory)]
#[Title("Halo 3: ODST")]
#[Build("13895.09.04.27.2201.atlas_release")]
#[Chunks(
    s_blf_chunk_start_of_file,
    s_blf_chunk_author,
    s_blf_chunk_end_of_file,
    s_blf_chunk_message_of_the_day,
    s_blf_chunk_banhammer_messages,
    s_blf_chunk_map_manifest,
    s_blf_chunk_message_of_the_day_popup,
    s_blf_chunk_scenario,
    s_blf_chunk_online_file_manifest,
    s_blf_chunk_network_configuration,
)]
pub struct v13895_09_04_27_2201_atlas_release {}