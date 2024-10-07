use std::ffi::c_char;
use blf_lib_derivable::blf::chunks::{BlfChunk, Serializable};
use blf_lib_derive::ChunkFactory;
use crate::blf::chunks::halo3;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::s_blf_chunk_start_of_file;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_author::s_blf_chunk_author;

#[derive(ChunkFactory)]
#[Title("Halo 3")]
#[Build("12070.08.09.05.2031.halo3_ship")]
#[Chunks(
    s_blf_chunk_start_of_file, 
    s_blf_chunk_author
)]
struct v12070_08_09_05_2031_halo3_ship {}
