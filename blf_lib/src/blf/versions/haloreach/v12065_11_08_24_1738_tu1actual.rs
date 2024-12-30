use blf_lib_derive::ChunkFactory;
use crate::blf::chunks::halo3;
use crate::blf::chunks::haloreach;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_author::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_player_data::*;


#[derive(ChunkFactory)]
#[Title("Halo: Reach")]
#[Build("12065.11.08.24.1738.tu1actual")]
#[Chunks(
    s_blf_chunk_start_of_file,
    s_blf_chunk_author,
    s_blf_chunk_end_of_file,
    s_blf_chunk_player_data
)]
pub struct v12065_11_08_24_1738_tu1actual {}