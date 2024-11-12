use std::error::Error;
use std::io::Write;
use serde::{Deserialize, Serialize};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_message_of_the_day_popup, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v13895_09_04_27_2201_atlas_release;
use blf_lib::blf_file;
use blf_lib::io::read_json_file;
use crate::build_path;
use crate::io::create_parent_folders;

pub const k_motd_popup_file_name: &str = "black_motd_popup.bin";
pub const k_vidmaster_popup_file_name: &str = "black_vidmaster_popup.bin";
pub const k_motd_popup_image_file_name: &str = "black_motd_popup_image.jpg";
pub const k_vidmaster_popup_image_file_name: &str = "black_vidmaster_popup_image.jpg";
pub const k_motd_popup_config_folder: &str = "popup";
pub const k_vidmaster_popup_config_folder: &str = "popup_vidmaster";

blf_file! {
    pub struct motd_popup {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mtdp: s_blf_chunk_message_of_the_day_popup,
        _eof: s_blf_chunk_end_of_file,
    }
}

#[derive(Serialize, Deserialize)]
pub struct motd_popup_config {
    pub motdIdentifier: u32,
    pub acceptWaitMilliseconds: u32,
    pub title: String,
    pub heading: String,
    pub accept: String,
    pub wait: String,
    pub body: String,
}

impl motd_popup {
    pub fn create(mtdp: s_blf_chunk_message_of_the_day_popup) -> Self {
        Self {
            _blf: s_blf_chunk_start_of_file::default(),
            athr: s_blf_chunk_author::for_build::<v13895_09_04_27_2201_atlas_release>(),
            mtdp,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }

    pub fn read_from_config(
        hoppers_config_folder: impl Into<String>,
        language_code: &str,
        vidmaster: bool
    ) -> Result<Self, Box<dyn Error>> {
        let motd_popup_json_path = build_path!(
            hoppers_config_folder,
            { if vidmaster { k_vidmaster_popup_config_folder } else { k_motd_popup_config_folder } },
            format!("{language_code}.json")
        );

        let config = read_json_file::<motd_popup_config>(&motd_popup_json_path)?;

        Ok(Self::create(s_blf_chunk_message_of_the_day_popup::create(
            config.motdIdentifier,
            config.acceptWaitMilliseconds,
            config.title,
            config.heading,
            config.accept,
            config.wait,
            config.body,
        )?))
    }

    pub fn write_to_config(&self, hoppers_config_path: &String, language_code: &str, vidmaster: bool) -> Result<(), Box<dyn Error>> {
        let config_file_path = build_path!(
            hoppers_config_path,
            { if vidmaster { k_vidmaster_popup_config_folder} else { k_motd_popup_config_folder } },
            format!("{language_code}.json")
        );

        let config = motd_popup_config {
            motdIdentifier: self.mtdp.title_index_identifier,
            acceptWaitMilliseconds: self.mtdp.button_key_wait_time_ms,
            title: self.mtdp.title.get_string(),
            heading: self.mtdp.header.get_string(),
            accept: self.mtdp.button_key.get_string(),
            wait: self.mtdp.button_key_wait.get_string(),
            body: self.mtdp.message.get_string(),
        };

        create_parent_folders(&config_file_path)?;
        let motd_json = serde_json::to_string_pretty(&config).unwrap();
        let mut text_file = File::create(config_file_path).unwrap();
        text_file.write_all(motd_json.as_bytes())?;

        Ok(())
    }}