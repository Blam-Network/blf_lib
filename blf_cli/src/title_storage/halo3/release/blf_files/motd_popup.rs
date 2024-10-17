use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_message_of_the_day_popup, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::little_endian;

blf_file! {
    pub struct motd_popup {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mtdp: s_blf_chunk_message_of_the_day_popup,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl motd_popup {
    pub fn create(mtdp: s_blf_chunk_message_of_the_day_popup) -> Self {
        Self {
            _blf: s_blf_chunk_start_of_file::new("halo3 motd", little_endian),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            mtdp,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}