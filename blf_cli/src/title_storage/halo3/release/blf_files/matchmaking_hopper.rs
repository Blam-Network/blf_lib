use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_hopper_configuration_table, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;

pub const k_matchmaking_hopper_file_name: &str = "matchmaking_hopper_011.bin";

blf_file! {
    pub struct matchmaking_hopper {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mhcf: s_blf_chunk_hopper_configuration_table,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl matchmaking_hopper {
    pub fn create(hopper_table: s_blf_chunk_hopper_configuration_table) -> matchmaking_hopper {
        matchmaking_hopper {
            _blf: s_blf_chunk_start_of_file::new("hopper config", byte_order_mark::default()),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            mhcf: hopper_table,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}