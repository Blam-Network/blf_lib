use blf_lib::blf::chunks::TitleAndBuild;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_banhammer_messages, s_blf_chunk_end_of_file, s_blf_chunk_message_of_the_day, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::{byte_order_mark, little_endian};

blf_file! {
    pub struct motd {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        motd: s_blf_chunk_message_of_the_day,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl motd {
    pub fn create(motd: String) -> Self {
        Self {
            _blf: s_blf_chunk_start_of_file::new("halo3 motd", little_endian),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            motd: s_blf_chunk_message_of_the_day::new(motd),
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}