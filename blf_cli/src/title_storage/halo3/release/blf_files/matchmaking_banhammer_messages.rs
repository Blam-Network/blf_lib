use std::error::Error;
use std::io::{Read, Write};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_banhammer_messages, s_blf_chunk_end_of_file, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use crate::build_path;
use crate::io::create_parent_folders;
use std::fs::File;

pub const k_matchmaking_banhammer_messages_file_name: &str = "matchmaking_banhammer_messages.bin";
pub const k_matchmaking_banhammer_messages_config_folder_name: &str = "banhammer_messages";

blf_file! {
    pub struct matchmaking_banhammer_messages {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        bhms: s_blf_chunk_banhammer_messages,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl matchmaking_banhammer_messages {
    fn create(banhammer_messages: Vec<String>) -> matchmaking_banhammer_messages {
        matchmaking_banhammer_messages {
            _blf: s_blf_chunk_start_of_file::default(),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            bhms: s_blf_chunk_banhammer_messages::create(banhammer_messages),
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }

    pub fn read_from_config(
        hoppers_config_path: &String,
        language_code: &str,
    ) -> Result<matchmaking_banhammer_messages, Box<dyn Error>> {
        let config_file_path = build_path!(
            hoppers_config_path,
            k_matchmaking_banhammer_messages_config_folder_name,
            &format!("{language_code}.txt")
        );

        let mut config_file = File::open(config_file_path)?;
        let mut matchmaking_banhammer_messages: String = String::new();
        config_file.read_to_string(&mut matchmaking_banhammer_messages).unwrap();
        let matchmaking_banhammer_messages = matchmaking_banhammer_messages
            .lines()
            .map(String::from)
            .collect();
        Ok(matchmaking_banhammer_messages::create(matchmaking_banhammer_messages))
    }

    pub fn write_to_config(&self, hoppers_config_path: &String, language_code: &str) -> Result<(), Box<dyn Error>> {
        let config_file_path = build_path!(
            hoppers_config_path,
            k_matchmaking_banhammer_messages_config_folder_name,
            format!("{language_code}.txt")
        );

        let messages_text = self.bhms.get_messages()
            .join("\r\n");

        create_parent_folders(&config_file_path)?;

        let mut text_file = File::create(config_file_path).unwrap();

        text_file.write_all(messages_text.as_bytes())?;

        Ok(())
    }
}