use std::error::Error;
use std::path::Path;
use blf_lib::blf::get_blf_file_hash;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_end_of_file, s_blf_chunk_online_file_manifest, s_blf_chunk_start_of_file};
use blf_lib::blf_file;
use crate::io::build_path;
use crate::title_storage::halo3::release::blf_files::matchmaking_banhammer_messages::k_matchmaking_banhammer_messages_file_name;
use crate::title_storage::halo3::release::blf_files::matchmaking_hopper::k_matchmaking_hopper_file_name;
use crate::title_storage::halo3::release::blf_files::matchmaking_hopper_descriptions::k_matchmaking_hopper_descriptions_file_name;
use crate::title_storage::halo3::release::blf_files::matchmaking_tips::k_matchmaking_tips_file_name;
use crate::title_storage::halo3::release::blf_files::rsa_manifest::k_rsa_manifest_file_name;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::k_language_suffixes;

pub const k_manifest_file_name: &str = "manifest_001.bin";

blf_file! {
    pub struct manifest {
        _blf: s_blf_chunk_start_of_file,
        onfm: s_blf_chunk_online_file_manifest,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl manifest {
    pub fn create(onfm: s_blf_chunk_online_file_manifest) -> Self {
        Self {
            _blf: s_blf_chunk_start_of_file::default(),
            onfm,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }

    pub fn build_for_hoppers<T: BlfChunk>(hoppers_blf_folder: &String) -> Result<manifest, Box<dyn Error>> {
        let mut manifest_chunk = s_blf_chunk_online_file_manifest::default();
        let hopper_directory_name = Path::new(hoppers_blf_folder).file_name().unwrap().to_str().unwrap();

        let network_configuration_file_name = format!("network_configuration_{:0>3}.bin", T::get_version().major);

        // h3 manifest includes:
        // - hopper config file
        // - network config
        // - rsa manifest
        // - banhammer messages
        // - hopper descriptions
        // - matchmaking tips

        let hopper_config_file_hash = get_blf_file_hash(build_path(vec![
            &hoppers_blf_folder,
            &k_matchmaking_hopper_file_name.to_string(),
        ]))?;

        let network_config_file_hash = get_blf_file_hash(build_path(vec![
            &hoppers_blf_folder,
            &network_configuration_file_name,
        ]))?;

        let rsa_manifest_file_hash = get_blf_file_hash(build_path(vec![
            &hoppers_blf_folder,
            &k_rsa_manifest_file_name.to_string(),
        ]))?;

        manifest_chunk.add_file_hash(
            format!("/title/{hopper_directory_name}/{k_matchmaking_hopper_file_name}"),
            hopper_config_file_hash,
        )?;

        manifest_chunk.add_file_hash(
            format!("/title/{hopper_directory_name}/{network_configuration_file_name}"),
            network_config_file_hash,
        )?;

        manifest_chunk.add_file_hash(
            format!("/title/{hopper_directory_name}/{k_rsa_manifest_file_name}"),
            rsa_manifest_file_hash,
        )?;

        for language_code in k_language_suffixes {
            let banhammer_messages_file_hash = get_blf_file_hash(build_path(vec![
                &hoppers_blf_folder,
                &language_code.to_string(),
                &k_matchmaking_banhammer_messages_file_name.to_string(),
            ]))?;

            let hopper_descriptions_file_hash = get_blf_file_hash(build_path(vec![
                &hoppers_blf_folder,
                &language_code.to_string(),
                &k_matchmaking_hopper_descriptions_file_name.to_string(),
            ]))?;

            let matchmaking_tips_file_hash = get_blf_file_hash(build_path(vec![
                &hoppers_blf_folder,
                &language_code.to_string(),
                &k_matchmaking_tips_file_name.to_string(),
            ]))?;

            manifest_chunk.add_file_hash(
                format!("/title/{hopper_directory_name}/{language_code}/{k_matchmaking_banhammer_messages_file_name}"),
                banhammer_messages_file_hash,
            )?;

            manifest_chunk.add_file_hash(
                format!("/title/{hopper_directory_name}/{language_code}/{k_matchmaking_hopper_descriptions_file_name}"),
                hopper_descriptions_file_hash,
            )?;

            manifest_chunk.add_file_hash(
                format!("/title/{hopper_directory_name}/{language_code}/{k_matchmaking_tips_file_name}"),
                matchmaking_tips_file_hash,
            )?;
        }

        Ok(Self::create(manifest_chunk))
    }
}