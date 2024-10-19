use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_banhammer_messages, s_blf_chunk_end_of_file, s_blf_chunk_map_variant, s_blf_chunk_packed_map_variant, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;

blf_file! {
    pub struct map_variant {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mpvr: s_blf_chunk_packed_map_variant,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl map_variant {
    pub fn create(map_variant: c_map_variant) -> map_variant {
        map_variant {
            _blf: s_blf_chunk_start_of_file::default(),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            mpvr: s_blf_chunk_packed_map_variant::create(map_variant),
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}