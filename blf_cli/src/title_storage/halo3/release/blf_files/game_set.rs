use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_game_set, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;

pub const k_game_set_file_name: &str = "game_set_006.bin";

blf_file! {
    pub struct game_set {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        gset: s_blf_chunk_game_set,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl game_set {
    pub fn create(game_set: s_blf_chunk_game_set) -> game_set {
        game_set {
            _blf: s_blf_chunk_start_of_file::new("game set", byte_order_mark::default()),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            gset: game_set,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}