use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_network_configuration, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::little_endian;

blf_file! {
    pub struct network_configuration {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        netc: s_blf_chunk_network_configuration,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl network_configuration {
    pub fn create(netc: s_blf_chunk_network_configuration) -> Self {
        Self {
            _blf: s_blf_chunk_start_of_file::new("halo3 net config", little_endian),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            netc,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}