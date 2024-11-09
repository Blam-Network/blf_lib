use std::error::Error;
use std::io::Write;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_message_of_the_day, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v13895_09_04_27_2201_atlas_release;
use blf_lib::blf_file;
use blf_lib::io::read_file_to_string;
use crate::build_path;
use crate::io::create_parent_folders;

pub const k_motd_file_name: &str = "black_motd.bin";
pub const k_motd_image_file_name: &str = "black_motd_image.jpg";
pub const k_motd_config_folder: &str = "motd";

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
            _blf: s_blf_chunk_start_of_file::default(),
            athr: s_blf_chunk_author::for_build::<v13895_09_04_27_2201_atlas_release>(),
            motd: s_blf_chunk_message_of_the_day::new(motd),
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }

    pub fn read_from_config(
        hoppers_config_path: &String,
        language_code: &str,
    ) -> Result<motd, Box<dyn Error>> {
        Ok(motd::create(read_file_to_string(build_path!(
            hoppers_config_path,
            k_motd_config_folder,
            format!("{language_code}.txt")
        ))?))
    }

    pub fn write_to_config(&self, hoppers_config_path: &String, language_code: &str) -> Result<(), Box<dyn Error>> {
        let config_file_path = build_path!(
            hoppers_config_path,
            k_motd_config_folder,
            format!("{language_code}.txt")
        );

        let messages_text = self.motd.get_message();

        create_parent_folders(&config_file_path)?;

        let mut text_file = File::create(config_file_path).unwrap();

        text_file.write_all(messages_text.as_bytes())?;

        Ok(())
    }
}