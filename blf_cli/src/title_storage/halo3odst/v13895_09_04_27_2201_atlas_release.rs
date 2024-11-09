mod blf_files;

use std::error::Error;
use std::fs;
use std::time::SystemTime;
use crate::io::{get_directories_in_folder, FILE_SEPARATOR};
use crate::{build_path, debug_log, title_converter, やった};
use crate::title_storage::{check_file_exists, TitleConverter};
use inline_colorization::*;
use lazy_static::lazy_static;
use blf_lib::blam::common::cseries::language::{get_language_string, k_language_suffix_chinese_traditional, k_language_suffix_english, k_language_suffix_french, k_language_suffix_german, k_language_suffix_italian, k_language_suffix_japanese, k_language_suffix_korean, k_language_suffix_mexican, k_language_suffix_portuguese, k_language_suffix_spanish};
use blf_lib::blf::BlfFile;
use blf_lib::blf::chunks::find_chunk_in_file;
use crate::console::console_task;
use crate::title_storage::halo3::release::blf_files::matchmaking_banhammer_messages::{k_matchmaking_banhammer_messages_file_name, matchmaking_banhammer_messages};
use regex::Regex;
use tempdir::TempDir;
use crate::title_storage::halo3odst::v13895_09_04_27_2201_atlas_release::blf_files::manifest::{k_manifest_file_name, manifest};
use blf_files::network_configuration::network_configuration;
use blf_lib::blf::versions::halo3odst::v13895_09_04_27_2201_atlas_release::s_blf_chunk_network_configuration;
use crate::title_storage::halo3odst::v13895_09_04_27_2201_atlas_release::blf_files::k_hopper_directory_name_max_length;
use crate::title_storage::halo3odst::v13895_09_04_27_2201_atlas_release::blf_files::network_configuration::k_network_configuration_file_name;
use crate::title_storage::halo3odst::v13895_09_04_27_2201_atlas_release::blf_files::rsa_manifest::{k_rsa_manifest_file_name, rsa_manifest};
use crate::title_storage::halo3odst::v13895_09_04_27_2201_atlas_release::blf_files::motd::{k_motd_config_folder, k_motd_file_name, k_motd_image_file_name, motd};
use crate::title_storage::halo3odst::v13895_09_04_27_2201_atlas_release::blf_files::motd_popup::{k_motd_popup_config_folder, k_motd_popup_file_name, k_motd_popup_image_file_name, k_vidmaster_popup_config_folder, k_vidmaster_popup_file_name, k_vidmaster_popup_image_file_name, motd_popup};

pub const k_build_string_halo3_ship_12070: &str = "13895.09.04.27.2201.atlas_release";

title_converter! (
    #[Title("Halo 3: ODST")]
    #[Build("13895.09.04.27.2201.atlas_release")]
    pub struct v13895_09_04_27_2201_atlas_release {}
);

// Halo 3: ODST's xex supports 12 languages, but only 10 were released.
pub const k_language_suffixes: [&str; 10] = [
    k_language_suffix_english,
    k_language_suffix_japanese,
    k_language_suffix_german,
    k_language_suffix_french,
    k_language_suffix_spanish,
    k_language_suffix_mexican,
    k_language_suffix_italian,
    k_language_suffix_korean,
    k_language_suffix_chinese_traditional,
    // k_language_suffix_chinese_simplified,
    k_language_suffix_portuguese,
    // k_language_suffix_polish,
];

lazy_static! {
    static ref config_rsa_signature_file_map_id_regex: Regex = Regex::new(r"^[0-9]{1,}").unwrap();
}

impl TitleConverter for v13895_09_04_27_2201_atlas_release {
    fn build_blfs(&mut self, config_path: &String, blfs_path: &String) {
        let start_time = SystemTime::now();

        println!("{style_bold}Writing Title Storage BLFs to {blfs_path} {style_reset}");

        let hopper_directories = get_directories_in_folder(&config_path).unwrap_or_else(|err|{
            println!("{}", err);
            panic!()
        });

        for hopper_directory in hopper_directories {
            let result = || -> Result<(), Box<dyn Error>> {
                if hopper_directory.len() > k_hopper_directory_name_max_length {
                    return Err(Box::from(format!(
                        "Hoppers folder \"{hopper_directory}\" is too long and will be skipped. ({} > {} characters)",
                        hopper_directory.len(),
                        k_hopper_directory_name_max_length
                    )))
                }

                let build_temp_dir = TempDir::new("blf_cli")?;
                let build_temp_dir_path = String::from(build_temp_dir.path().to_str().unwrap());

                debug_log!("Using temp directory: {build_temp_dir_path}");

                let hopper_config_path = build_path!(
                    config_path,
                    &hopper_directory
                );

                let hopper_blfs_path = build_path!(
                    blfs_path,
                    &hopper_directory
                );

                println!("{style_bold}Converting {color_bright_white}{}{style_reset}...", hopper_directory);
                Self::build_blf_banhammer_messages(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_motds(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_motd_popups(&hopper_config_path, &hopper_blfs_path, false)?;
                Self::build_blf_motd_popups(&hopper_config_path, &hopper_blfs_path, true)?;
                Self::build_blf_map_manifest(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_network_configuration(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_manifest(&hopper_blfs_path)?;
                Ok(())
            }();

            if result.is_err() {
                println!("{color_red}Failed to build title storage for hoppers {hopper_directory}{style_reset}");
                println!("{color_red}{}{style_reset}", result.err().unwrap());
            }
        }

        let seconds = start_time.elapsed().unwrap().as_secs_f32();
        println!("Finished conversion in {seconds:.2} seconds.");
    }

    fn build_config(&mut self, blfs_path: &String, config_path: &String) {
        println!("{style_bold}Writing Title Storage config to {config_path} {style_reset}");

        let hopper_directories = get_directories_in_folder(&blfs_path).unwrap_or_else(|err|{
            println!("{}", err);
            panic!();
        });

        for hopper_directory in hopper_directories {
            let result = || -> Result<(), Box<dyn Error>> {
                if hopper_directory.len() > k_hopper_directory_name_max_length {
                    return Err(Box::<dyn Error>::from(format!("Skipping \"{hopper_directory}\" as it's name is too long. ({k_hopper_directory_name_max_length} characters MAX)")))
                }

                let hoppers_config_path = build_path!(
                    config_path,
                    &hopper_directory
                );

                let hoppers_blf_path = build_path!(
                    blfs_path,
                    &hopper_directory
                );

                println!("{style_bold}Converting {color_bright_white}{}{style_reset}...", hopper_directory);
                Self::build_config_banhammer_messages(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_motds(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_popups(&hoppers_blf_path, &hoppers_config_path, false)?;
                Self::build_config_popups(&hoppers_blf_path, &hoppers_config_path, true)?;
                Self::build_config_network_configuration(&hoppers_blf_path, &hoppers_config_path);
                Ok(())
            }();

            if result.is_err() {
                println!("{color_red}Failed to build title storage for hoppers {hopper_directory}{style_reset}");
                println!("{color_red}{}{style_reset}", result.err().unwrap());
            }
        }
    }
}

impl v13895_09_04_27_2201_atlas_release {
    fn build_config_banhammer_messages(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting Banhammer Messages");

        for language_code in k_language_suffixes {
            let blf_file_path = build_path!(hoppers_blf_path, language_code, k_matchmaking_banhammer_messages_file_name);

            if !check_file_exists(&blf_file_path) {
                task.add_warning(format!(
                    "No {} banhammer messages are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            matchmaking_banhammer_messages::read(&blf_file_path)?.write_to_config(hoppers_config_path, language_code)?;
        }

        やった!(task)
    }

    fn build_config_motds(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting MOTDs");

        // BLFs
        for language_code in k_language_suffixes {
            let file_path = build_path!(
                hoppers_blf_path,
                language_code,
                k_motd_file_name
            );


            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} MOTD is present.",
                    get_language_string(language_code)
                ));

                continue;
            }

            motd::read(&file_path)?.write_to_config(hoppers_config_path, language_code)?;
        }

        // JPEGs
        for language_code in k_language_suffixes {
            let file_path = build_path!(
                hoppers_blf_path,
                language_code,
                k_motd_image_file_name
            );

            let output_path = build_path!(
                hoppers_config_path,
                k_motd_config_folder,
                format!("{language_code}.jpg")
            );

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} MOTD image is present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            fs::copy(file_path, output_path)?;
        }

        やった!(task)
    }

    fn build_config_popups(hoppers_blf_path: &String, hoppers_config_path: &String, vidmaster: bool) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start(
            if vidmaster { "Converting Vidmaster Popups" }
            else { "Converting MOTD Popups" }
        );

        // BLFs
        for language_code in k_language_suffixes {
            let file_path = build_path!(
                hoppers_blf_path,
                language_code,
                if vidmaster { k_vidmaster_popup_file_name } else { k_motd_popup_file_name }
            );

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {} Popup is present.",
                    get_language_string(language_code),
                    if vidmaster { "Vidmaster " } else { "MOTD" }
                ));

                continue;
            }

            motd_popup::read(&file_path)?.write_to_config(hoppers_config_path, language_code, vidmaster)?;
        }

        // JPEGs
        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}{FILE_SEPARATOR}{}motd_popup_image.jpg", if vidmaster { "blue_" } else { "" });
            let file_path = format!("{hoppers_blf_path}{FILE_SEPARATOR}{relative_file_path}");
            let output_path = build_path!(
                hoppers_blf_path,
                language_code,
                format!("{language_code}.jpg")
            );

            let file_path = build_path!(
                hoppers_blf_path,
                language_code,
                if vidmaster { k_vidmaster_popup_image_file_name } else { k_motd_popup_image_file_name }
            );

            let output_path = build_path!(
                hoppers_config_path,
                if vidmaster { k_vidmaster_popup_config_folder } else { k_motd_popup_config_folder },
                format!("{language_code}.jpg")
            );

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {} Popup image is present.",
                    get_language_string(language_code),
                    if vidmaster { "Vidmaster " } else { "MOTD" }
                ));

                continue;
            }

            fs::copy(file_path, output_path).unwrap();
        }

        やった!(task)
    }

    fn build_config_network_configuration(hoppers_blfs_path: &String, hoppers_config_path: &String) {
        // For now we just copy it as is. But we do check that it contains a netc.
        let mut task = console_task::start("Converting Network Configuration");

        let network_configuration_source_path = build_path!(
            hoppers_blfs_path,
            k_network_configuration_file_name
        );

        let network_configuration_dest_path = build_path!(
            hoppers_config_path,
            k_network_configuration_file_name
        );

        // We read and rewrite to tidy any padding and the headers.
        let mut network_config = network_configuration::read(&network_configuration_source_path).unwrap();
        network_config.write(&network_configuration_dest_path);

        fs::copy(network_configuration_source_path, network_configuration_dest_path).unwrap();

        task.complete();
    }

    fn build_blf_banhammer_messages(hoppers_config_folder: &String, hoppers_blf_folder: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Banhammer Messages");

        for language_code in k_language_suffixes {
            let matchmaking_banhammer_messages = matchmaking_banhammer_messages::read_from_config(
                hoppers_config_folder,
                language_code,
            );

            if matchmaking_banhammer_messages.is_err() {
                task.add_warning(format!("Failed to build {} banhammer messages.", get_language_string(language_code)));
                continue;
            }

            matchmaking_banhammer_messages?.write(build_path!(
                hoppers_blf_folder,
                language_code,
                k_matchmaking_banhammer_messages_file_name
            ));
        }

        やった!(task)
    }

    fn build_blf_motds(
        hoppers_config_path: &String,
        hoppers_blf_path: &String,
    ) -> Result<(), Box<dyn Error>>
    {
        let mut task = console_task::start("Building MOTDs");

        for language_code in k_language_suffixes {
            let motd = motd::read_from_config(
                hoppers_config_path,
                language_code
            );

            if motd.is_err() {
                task.add_warning(format!(
                    "Failed to build {} MOTD: {}",
                    get_language_string(language_code),
                    motd.unwrap_err()
                ));

                continue;
            }

            motd?.write(build_path!(
                hoppers_blf_path,
                language_code,
                k_motd_file_name
            ));
        }

        for language_code in k_language_suffixes {
            let jpeg_file_path = build_path!(
                hoppers_config_path,
                k_motd_config_folder,
                format!("{}.jpg", language_code)
            );

            let destination_path = build_path!(
                hoppers_blf_path,
                language_code,
                k_motd_image_file_name
            );

            if !check_file_exists(&jpeg_file_path) {
                task.add_warning(format!(
                    "No {} MOTD Image is present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            fs::copy(jpeg_file_path, destination_path).unwrap();
        }

        やった!(task)
    }

    fn build_blf_motd_popups(hoppers_config_folder: &String, hoppers_blf_folder: &String, vidmaster: bool) -> Result<(), Box<dyn Error>>{
        let mut task = console_task::start(format!(
            "Building {} Popups",
            if vidmaster { "Vidmaster" } else { "MOTD" }
        ));

        for language_code in k_language_suffixes {
            let motd_popup = motd_popup::read_from_config(hoppers_config_folder, language_code, vidmaster);

            if motd_popup.is_err() {
                task.add_warning(format!(
                    "Failed to build {} {} Popup: {}",
                    get_language_string(language_code),
                    if vidmaster { "Vidmaster" } else { "MOTD" },
                    motd_popup.unwrap_err()
                ));

                continue;
            }

            motd_popup?.write(build_path!(
                hoppers_blf_folder,
                language_code,
                if vidmaster { k_vidmaster_popup_file_name } else { k_motd_popup_file_name }
            ));
        }

        for language_code in k_language_suffixes {
            let jpeg_file_path = build_path!(
                hoppers_config_folder,
                if vidmaster { k_vidmaster_popup_config_folder } else { k_motd_popup_config_folder },
                format!("{}.jpg", language_code)
            );

            let destination_path = build_path!(
                hoppers_blf_folder,
                language_code,
                if vidmaster { k_vidmaster_popup_image_file_name } else { k_motd_popup_image_file_name }
            );

            if !check_file_exists(&jpeg_file_path) {
                task.add_warning(format!(
                    "No {} {} Popup Image is present.",
                    get_language_string(language_code),
                    if vidmaster { "Vidmaster" } else { "MOTD" }
                ));

                continue;
            }

            fs::copy(jpeg_file_path, destination_path).unwrap();
        }

        やった!(task)
    }

    fn build_blf_map_manifest(hoppers_config_path: &String, hoppers_blf_path: &String) -> Result<(), Box<dyn Error>>
    {
        let mut task = console_task::start("Building Map Manifest");

        let mut rsa_manifest = rsa_manifest::build_for_hoppers(hoppers_config_path)
            .inspect_err(|_| { task.fail() })?;

        rsa_manifest.write(build_path!(
            hoppers_blf_path,
            k_rsa_manifest_file_name
        ));

        やった!(task)
    }

    fn build_blf_network_configuration(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
    ) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Network Configuration");
        let netc = find_chunk_in_file(build_path!(
            hoppers_config_path,
            k_network_configuration_file_name
        ))?;

        let mut network_configuration_blf_file = network_configuration::create(netc);
        network_configuration_blf_file.write(
            build_path!(
                hoppers_blfs_path,
                k_network_configuration_file_name
            )
        );

        やった!(task)
    }

    fn build_blf_manifest(
        hoppers_blfs_path: &String,
    ) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Manifest File");

        let mut manifest_blf_file = manifest::build_for_hoppers::<s_blf_chunk_network_configuration>(hoppers_blfs_path).inspect_err(|err|{
            task.fail();
        })?;

        manifest_blf_file.write(build_path!(
            hoppers_blfs_path,
            k_manifest_file_name
        ));

        やった!(task)
    }
}