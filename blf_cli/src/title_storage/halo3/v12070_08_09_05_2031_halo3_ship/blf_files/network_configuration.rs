use std::error::Error;
use std::io::Write;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_network_configuration, s_blf_chunk_start_of_file, s_network_configuration};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::io::read_json_file;
use blf_lib::types::byte_order_mark::byte_order_mark;
use crate::build_path;
use crate::io::create_parent_folders;
use std::fs::File;

pub const k_network_configuration_file_name: &str = "network_configuration_135.bin";
pub const k_network_configuration_json_file_name: &str = "network_configuration_135.json";

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
            _blf: s_blf_chunk_start_of_file::new("halo3 net config", byte_order_mark::little_endian),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            netc,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }

    pub fn write_to_config(&self, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let config_file_path = build_path!(
            hoppers_config_path,
            k_network_configuration_json_file_name
        );

        create_parent_folders(&config_file_path)?;

        let netc_json = serde_json::to_string_pretty(&self.netc.config).unwrap();
        let mut text_file = File::create(config_file_path).unwrap();
        text_file.write_all(netc_json.as_bytes())?;

        Ok(())
    }

    pub fn read_from_config(hoppers_config_path: &String) -> Result<Self, Box<dyn Error>> {
        let config_file_path = build_path!(
            hoppers_config_path,
            k_network_configuration_json_file_name
        );

        let network_config = read_json_file::<s_network_configuration>(config_file_path)?;
        let network_config = s_blf_chunk_network_configuration {
            config: network_config
        };

        Ok(Self::create(network_config))
    }
}