use std::error::Error;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_message_of_the_day, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::io::read_file_to_string;
use blf_lib::types::byte_order_mark::little_endian;
use crate::build_path;

pub const k_motd_file_name: &str = "motd.bin";
pub const k_mythic_motd_file_name: &str = "blue_motd.bin";
pub const k_motd_image_file_name: &str = "motd_image.bin";
pub const k_mythic_motd_image_file_name: &str = "blue_motd_image.bin";


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

    pub fn build_motd_for_language(
        hoppers_config_path: &String,
        language_code: &str,
        mythic: bool,
    ) -> Result<motd, Box<dyn Error>> {
        Ok(motd::create(read_file_to_string(build_path!(
            hoppers_config_path,
            if mythic { "motd_mythic" } else { "motd" },
            format!("{language_code}.txt")
        ))?))
    }
}