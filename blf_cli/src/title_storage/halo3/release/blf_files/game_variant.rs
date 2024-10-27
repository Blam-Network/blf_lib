use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_packed_game_variant, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;

blf_file! {
    pub struct game_variant {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        gvar: s_blf_chunk_packed_game_variant,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl game_variant {
    pub fn create(game_variant: c_game_variant) -> game_variant {
        game_variant {
            _blf: s_blf_chunk_start_of_file::new("map variant", byte_order_mark::default()),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            gvar: s_blf_chunk_packed_game_variant::create(game_variant),
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}