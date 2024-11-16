use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use serde::{Deserialize, Serialize};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{c_hopper_configuration, s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_hopper_configuration_table, s_blf_chunk_start_of_file, s_game_hopper_custom_category};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;
use crate::build_path;
use crate::io::ordered_map;

pub const k_matchmaking_hopper_file_name: &str = "matchmaking_hopper_011.bin";
pub const k_hoppers_config_folder_name: &str = "hoppers";
pub const k_active_hoppers_config_file_name: &str = "active_hoppers.txt";
pub const k_categories_config_file_name: &str = "categories.json";
pub const k_hopper_config_file_name: &str = "configuration.json";

blf_file! {
    pub struct matchmaking_hopper {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mhcf: s_blf_chunk_hopper_configuration_table,
        _eof: s_blf_chunk_end_of_file,
    }
}

#[derive(Serialize, Deserialize)]
pub struct matchmaking_hopper_config {
    #[serde(serialize_with = "ordered_map")]
    pub descriptions: HashMap<String, String>,
    pub configuration: c_hopper_configuration,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct matchmaking_hopper_category_configuration_and_descriptions {
    pub configuration: s_game_hopper_custom_category,
    #[serde(serialize_with = "ordered_map")]
    pub descriptions: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct matchmaking_hopper_categories_config {
    pub categories: Vec<matchmaking_hopper_category_configuration_and_descriptions>,
}

impl matchmaking_hopper_categories_config {
    pub fn read(hopper_config_folder: &String) -> Result<Self, Box<dyn Error>> {
        let config_file_path = build_path!(
            hopper_config_folder,
            k_categories_config_file_name
        );

        let config_file = File::open(&config_file_path)?;
        Ok(serde_json::from_reader(config_file)?)
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

pub fn read_active_hoppers(hoppers_config_folder: &String) -> Result<Vec<String>, String> {
    let active_hoppers_file_path = build_path!(
        hoppers_config_folder,
        k_active_hoppers_config_file_name
    );

    let active_hoppers_file = File::open(&active_hoppers_file_path);
    if active_hoppers_file.is_err() {
        return Err(active_hoppers_file.unwrap_err().to_string());
    }

    let mut active_hoppers_file = active_hoppers_file.unwrap();
    let mut active_hoppers_string = String::new();
    let read_result = active_hoppers_file.read_to_string(&mut active_hoppers_string);
    if read_result.is_err() {
        return Err(read_result.unwrap_err().to_string());
    }

    let active_hopper_folders = active_hoppers_string.lines();

    Ok(active_hopper_folders.map(|thing|String::from(thing)).collect::<Vec<String>>())
}