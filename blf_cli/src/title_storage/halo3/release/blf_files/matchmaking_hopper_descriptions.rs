use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_hopper_description_table, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;

pub const k_matchmaking_hopper_descriptions_file_name: &str = "matchmaking_hopper_descriptions_003.bin";

blf_file! {
    pub struct matchmaking_hopper_descriptions {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mhdf: s_blf_chunk_hopper_description_table,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl matchmaking_hopper_descriptions {
    pub fn create(descriptions_table: s_blf_chunk_hopper_description_table) -> matchmaking_hopper_descriptions {
        matchmaking_hopper_descriptions {
            _blf: s_blf_chunk_start_of_file::default(),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            mhdf: descriptions_table,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}