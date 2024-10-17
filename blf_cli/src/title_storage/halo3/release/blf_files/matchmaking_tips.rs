use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_banhammer_messages, s_blf_chunk_end_of_file, s_blf_chunk_matchmaking_tips, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;

blf_file! {
    pub struct matchmaking_tips {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mmtp: s_blf_chunk_matchmaking_tips,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl matchmaking_tips {
    pub fn create(tips: Vec<String>) -> matchmaking_tips {
        matchmaking_tips {
            _blf: s_blf_chunk_start_of_file::default(),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            mmtp: s_blf_chunk_matchmaking_tips::create(tips),
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}