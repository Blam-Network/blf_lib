use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::fs::{create_dir_all, exists, remove_file, File};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use crate::io::{build_path, get_directories_in_folder, get_files_in_folder, FILE_SEPARATOR};
use crate::title_converter;
use crate::title_storage::{check_file_exists, TitleConverter};
use inline_colorization::*;
use lazy_static::lazy_static;
use blf_lib::blam::common::cseries::language::{get_language_string, k_language_suffix_chinese_traditional, k_language_suffix_english, k_language_suffix_french, k_language_suffix_german, k_language_suffix_italian, k_language_suffix_japanese, k_language_suffix_korean, k_language_suffix_mexican, k_language_suffix_portuguese, k_language_suffix_spanish};
use blf_lib::blf::{get_blf_file_hash, BlfFile};
use blf_lib::blf::chunks::find_chunk_in_file;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_banhammer_messages, s_blf_chunk_game_set, s_blf_chunk_game_set_entry, s_blf_chunk_hopper_description_table, s_blf_chunk_map_manifest, s_blf_chunk_matchmaking_tips, s_blf_chunk_message_of_the_day, s_blf_chunk_message_of_the_day_popup, s_blf_chunk_packed_game_variant, s_blf_chunk_packed_map_variant};
use crate::console::console_task;
use crate::title_storage::halo3::release::blf_files::{motd, rsa_manifest};
use crate::title_storage::halo3::release::config_files::motd_popup::motd_popup as motd_popup_config;
use crate::title_storage::halo3::release::blf_files::motd_popup::motd_popup as motd_popup_blf;
use crate::title_storage::halo3::release::blf_files::matchmaking_banhammer_messages::{matchmaking_banhammer_messages as matchmaking_banhammer_messages_blf};
use crate::title_storage::halo3::release::blf_files::matchmaking_tips::matchmaking_tips as matchmaking_tips_blf;
use regex::Regex;
use tempdir::TempDir;
use tokio::runtime;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use blf_lib::blam::common::memory::crc::crc32;
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_hopper_configuration_table;
use blf_lib::types::byte_limited_utf8_string::StaticString;
use crate::title_storage::halo3::release::blf_files;
use crate::title_storage::halo3::release::blf_files::game_variant::game_variant;
use crate::title_storage::halo3::release::blf_files::map_variant::map_variant;
use crate::title_storage::halo3::release::blf_files::network_configuration::network_configuration;
use crate::title_storage::halo3::release::config_files::active_hoppers::read_active_hoppers;
use crate::title_storage::halo3::release::config_files::game_set::{build_game_set_csv, game_set};
use crate::title_storage::halo3::release::config_files::hopper_configuration::hopper_configuration as json_hopper_configuration;
use crate::title_storage::halo3::release::config_files::categories_configuration::{categories_configuration as json_categories_configuration, category_configuration_and_descriptions};

pub const k_build_string_halo3_ship_12070: &str = "12070.08.09.05.2031.halo3_ship";

title_converter! (
    #[Title("Halo 3")]
    #[Build("12070.08.09.05.2031.halo3_ship")]
    pub struct v12070_08_09_05_2031_halo3_ship {}
);

const HOPPER_DIRECTORY_NAME_MAX_LENGTH: usize = 64;

impl TitleConverter for v12070_08_09_05_2031_halo3_ship {
    fn build_blfs(&mut self, config_path: &String, blfs_path: &String) {
        println!("{style_bold}Writing Title Storage BLFs to {blfs_path} {style_reset}");

        let hopper_directories = get_directories_in_folder(&config_path).unwrap_or_else(|err|{
            println!("{}", err);
            panic!()
        });

        for hopper_directory in hopper_directories {
            if hopper_directory.len() > HOPPER_DIRECTORY_NAME_MAX_LENGTH {
                println!("{color_bright_white}{bg_red}Skipping \"{hopper_directory}\" as it's name is too long. ({HOPPER_DIRECTORY_NAME_MAX_LENGTH} characters MAX){style_reset}");
                continue;
            }

            let build_temp_dir = TempDir::new("blf_cli").unwrap();
            let build_temp_dir_path = String::from(build_temp_dir.path().to_str().unwrap());

            println!("Using temp directory: {build_temp_dir_path}");

            let hopper_config_path = build_path(vec![
                &config_path,
                &hopper_directory,
            ]);

            let hopper_blfs_path = build_path(vec![
                &blfs_path,
                &hopper_directory,
            ]);

            println!("{style_bold}Converting {color_bright_white}{}{style_reset}...", hopper_directory);
            Self::build_blf_banhammer_messages(config_path, &hopper_directory, blfs_path);
            Self::build_blf_matchmaking_tips(config_path, &hopper_directory, blfs_path);
            Self::build_blf_motds(config_path, &hopper_directory, blfs_path, false);
            Self::build_blf_motds(config_path, &hopper_directory, blfs_path, true);
            Self::build_blf_motd_popups(config_path, &hopper_directory, blfs_path, false);
            Self::build_blf_motd_popups(config_path, &hopper_directory, blfs_path, true);
            Self::build_blf_map_manifest(config_path, &hopper_directory, blfs_path);

            let active_hoppers = Self::read_active_hopper_configuration(&hopper_config_path);
            let game_sets = Self::read_game_set_configuration(&hopper_config_path, &active_hoppers);
            let mut game_variant_hashes = HashMap::<String, s_network_http_request_hash>::new();
            let mut map_variant_hashes = HashMap::<String, s_network_http_request_hash>::new();
            let mut map_variant_map_ids = HashMap::<String, u32>::new();

            Self::build_blf_game_variants(&hopper_config_path, &hopper_blfs_path, &build_temp_dir_path, &game_sets, &mut game_variant_hashes);
            Self::build_blf_map_variants(&hopper_config_path, &hopper_blfs_path, &build_temp_dir_path, &game_sets, &mut map_variant_hashes, &mut map_variant_map_ids);
            Self::build_blf_game_sets(&hopper_blfs_path, game_sets, &game_variant_hashes, &map_variant_hashes, &map_variant_map_ids, &build_temp_dir_path);

        }
    }

    fn build_config(&mut self, blfs_path: &String, config_path: &String) {
        println!("{style_bold}Writing Title Storage config to {config_path} {style_reset}");

        let hopper_directories = get_directories_in_folder(&blfs_path).unwrap_or_else(|err|{
            println!("{}", err);
            panic!();
        });

        for hopper_directory in hopper_directories {
            if hopper_directory.len() > HOPPER_DIRECTORY_NAME_MAX_LENGTH {
                println!("{color_bright_white}{bg_red}Skipping \"{hopper_directory}\" as it's name is too long. ({HOPPER_DIRECTORY_NAME_MAX_LENGTH} characters MAX){style_reset}");
                continue;
            }

            let hopper_config_path = build_path(vec![
                &config_path,
                &hopper_directory,
            ]);

            let hopper_blfs_path = build_path(vec![
                &blfs_path,
                &hopper_directory,
            ]);

            println!("{style_bold}Converting {color_bright_white}{}{style_reset}...", hopper_directory);
            Self::build_config_banhammer_messages(blfs_path, &hopper_directory, config_path);
            Self::build_config_matchmaking_tips(blfs_path, &hopper_directory, config_path);
            Self::build_config_motds(blfs_path, &hopper_directory, config_path, false);
            Self::build_config_motds(blfs_path, &hopper_directory, config_path, true);
            Self::build_config_motd_popups(blfs_path, &hopper_directory, config_path, false);
            Self::build_config_motd_popups(blfs_path, &hopper_directory, config_path, true);
            Self::build_config_map_variants(blfs_path, &hopper_directory, config_path);
            Self::build_config_game_variants(blfs_path, &hopper_directory, config_path);
            Self::build_config_game_sets(blfs_path, &hopper_directory, config_path);
            Self::build_config_hoppers(&hopper_blfs_path, &hopper_config_path);
            Self::build_config_network_configuration(&hopper_blfs_path, &hopper_config_path);
        }
    }
}

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
    static ref hopper_folder_regex: Regex = Regex::new(r"^[0-9]{5}.*").unwrap();
    static ref config_hopper_folder_identifier_regex: Regex = Regex::new(r"^[0-9]{1,5}").unwrap();
    static ref map_variant_regex: Regex = Regex::new(r"_012.bin$").unwrap();
    static ref game_variant_regex: Regex = Regex::new(r"_010.bin$").unwrap();
    static ref config_rsa_signature_file_map_id_regex: Regex = Regex::new(r"^[0-9]{1,}").unwrap();
}

impl v12070_08_09_05_2031_halo3_ship {
    fn build_config_banhammer_messages(blfs_path: &String, hopper_directory: &String, config_path: &String) {
        let mut task = console_task::start(String::from("Converting Banhammer Messages"));

        let banhammer_messages_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from("banhammer_messages"),
        ]);

        create_dir_all(&banhammer_messages_folder).unwrap();

        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}{FILE_SEPARATOR}matchmaking_banhammer_messages.bin");
            let file_path = format!("{blfs_path}{FILE_SEPARATOR}{hopper_directory}{FILE_SEPARATOR}{relative_file_path}");

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} banhammer messages are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let banhammer_messages =
                find_chunk_in_file::<s_blf_chunk_banhammer_messages>(&file_path);

            if banhammer_messages.is_err() {
                task.fail(format!("Failed to read banhammer messages file at: {file_path}"));
                return;
            }

            let output_text_file_path = build_path(vec![
                &banhammer_messages_folder,
                &format!("{language_code}.txt")
            ]);

            let messages_text = banhammer_messages.unwrap().get_messages()
                .join("\r\n");

            let mut text_file = File::create(output_text_file_path).unwrap();

            text_file.write_all(messages_text.as_bytes()).unwrap()
        }

        task.complete();
    }

    fn build_config_matchmaking_tips(blfs_path: &String, hopper_directory: &String, config_path: &String) {
        let mut task = console_task::start(String::from("Converting Matchmaking Tips"));

        let banhammer_messages_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from("matchmaking_tips"),
        ]);

        create_dir_all(&banhammer_messages_folder).unwrap();

        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}{FILE_SEPARATOR}matchmaking_tips.bin");
            let file_path = format!("{blfs_path}{FILE_SEPARATOR}{hopper_directory}{FILE_SEPARATOR}{relative_file_path}");

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} banhammer messages are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let matchmaking_tips =
                find_chunk_in_file::<s_blf_chunk_matchmaking_tips>(&file_path);

            if matchmaking_tips.is_err() {
                task.fail(format!("Failed to read banhammer messages file at: {file_path}"));
                return;
            }

            let output_text_file_path = build_path(vec![
                &banhammer_messages_folder,
                &format!("{language_code}.txt")
            ]);

            let messages_text = matchmaking_tips.unwrap().get_tips()
                .join("\r\n");

            let mut text_file = File::create(output_text_file_path).unwrap();

            text_file.write_all(messages_text.as_bytes()).unwrap()
        }

        task.complete();
    }

    fn build_config_motds(blfs_path: &String, hopper_directory: &String, config_path: &String, mythic: bool) {
        let mut task = console_task::start(
            if mythic { String::from("Converting Mythic MOTDs") }
            else { String::from("Converting MOTDs") }
        );

        let motd_messages_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from(if mythic { "motd_mythic" } else { "motd" }),
        ]);

        create_dir_all(&motd_messages_folder).unwrap();

        // BLFs
        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}{FILE_SEPARATOR}{}motd.bin", if mythic { "blue_" } else { "" });
            let file_path = format!("{blfs_path}{FILE_SEPARATOR}{hopper_directory}{FILE_SEPARATOR}{relative_file_path}");

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            let motd =
                find_chunk_in_file::<s_blf_chunk_message_of_the_day>(&file_path);

            if motd.is_err() {
                task.fail(format!("Failed to read MOTD file at {file_path}"));
                return;
            }

            let motd = motd.unwrap();

            let output_text_file_path = build_path(vec![
                &motd_messages_folder,
                &format!("{language_code}.txt")
            ]);

            let message_text = motd.get_message();

            let mut text_file = File::create(output_text_file_path).unwrap();

            text_file.write_all(message_text.as_bytes()).unwrap()
        }

        // JPEGs
        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}{FILE_SEPARATOR}{}motd_image.jpg", if mythic { "blue_" } else { "" });
            let file_path = format!("{blfs_path}{FILE_SEPARATOR}{hopper_directory}{FILE_SEPARATOR}{relative_file_path}");
            let output_path = build_path(vec![
                &motd_messages_folder,
                &format!("{language_code}.jpg")
            ]);

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD image is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            std::fs::copy(file_path, output_path).unwrap();
        }

        task.complete();
    }

    fn build_config_motd_popups(blfs_path: &String, hopper_directory: &String, config_path: &String, mythic: bool) {
        let mut task = console_task::start(
            if mythic { String::from("Converting Mythic MOTD Popups") }
            else { String::from("Converting MOTD Popups") }
        );

        let motd_messages_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from(if mythic { "popup_mythic" } else { "popup" }),
        ]);

        create_dir_all(&motd_messages_folder).unwrap();

        // BLFs
        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}{FILE_SEPARATOR}{}motd_popup.bin", if mythic { "blue_" } else { "" });
            let file_path = format!("{blfs_path}{FILE_SEPARATOR}{hopper_directory}{FILE_SEPARATOR}{relative_file_path}");

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD Popup is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            let motd_popup_chunk =
                find_chunk_in_file::<s_blf_chunk_message_of_the_day_popup>(&file_path);

            if motd_popup_chunk.is_err() {
                task.fail(format!("Failed to read MOTD Popup file at {file_path}"));
                return;
            }

            let motd_popup_chunk = motd_popup_chunk.unwrap();
            let motd_popup_config = motd_popup_config::from_chunk(motd_popup_chunk);

            let output_text_file_path = build_path(vec![
                &motd_messages_folder,
                &format!("{language_code}.json")
            ]);

            let motd_json = serde_json::to_string_pretty(&motd_popup_config).unwrap();

            let mut json_file = File::create(output_text_file_path).unwrap();

            json_file.write_all(motd_json.as_bytes()).unwrap()
        }

        // JPEGs
        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}{FILE_SEPARATOR}{}motd_popup_image.jpg", if mythic { "blue_" } else { "" });
            let file_path = format!("{blfs_path}{FILE_SEPARATOR}{hopper_directory}{FILE_SEPARATOR}{relative_file_path}");
            let output_path = build_path(vec![
                &motd_messages_folder,
                &format!("{language_code}.jpg")
            ]);

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD Popup image is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            std::fs::copy(file_path, output_path).unwrap();
        }

        task.complete();
    }

    fn build_config_map_variants(blfs_path: &String, hopper_directory: &String, config_path: &String) {
        let mut task = console_task::start(String::from("Converting Map Variants"));

        let map_variants_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from("map_variants"),
        ]);

        create_dir_all(&map_variants_folder).unwrap();

        let current_hoppers_blf_folder = build_path(vec![
            blfs_path,
            hopper_directory,
        ]);

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(&current_hoppers_blf_folder).unwrap_or_else(|err|{
            println!("{}", err);
            panic!();
        });

        let mut converted_maps = Vec::<String>::new();

        for subfolder in hopper_directory_subfolders {
            if !hopper_folder_regex.is_match(&subfolder) {
                continue;
            }

            let map_variant_blfs_folder = build_path(vec![
                &current_hoppers_blf_folder,
                &subfolder,
                &String::from("map_variants"),
            ]);

            if !exists(&map_variant_blfs_folder).unwrap() {
                continue;
            }

            let map_variant_files = get_files_in_folder(&map_variant_blfs_folder).unwrap_or_else(|err|{
                println!("{}", err);
                panic!();
            });

            for map_variant_file_name in map_variant_files {
                if !map_variant_regex.is_match(&map_variant_file_name) {
                    continue;
                }

                let map_variant_blf_file_path = build_path(vec![
                    &map_variant_blfs_folder,
                    &map_variant_file_name,
                ]);

                let map_variant_json_file_path = build_path(vec![
                    &map_variants_folder,
                    &map_variant_file_name.replace("_012.bin", ".json"),
                ]);

                if converted_maps.contains(&map_variant_file_name) {
                    continue;
                }
                else {
                    converted_maps.push(map_variant_file_name);
                }

                if exists(&map_variant_json_file_path).unwrap() {
                    remove_file(&map_variant_json_file_path).unwrap()
                }

                let packed_map_variant = find_chunk_in_file::<s_blf_chunk_packed_map_variant>(&map_variant_blf_file_path).unwrap();
                let map_variant_json = serde_json::to_string_pretty(&packed_map_variant.map_variant).unwrap();
                let mut map_variant_json_file = File::create_new(map_variant_json_file_path).unwrap();
                map_variant_json_file.write_all(map_variant_json.as_bytes()).unwrap();
            }
        }

        task.add_message(format!("Converted {} map variants.", converted_maps.len()));

        task.complete();
    }

    fn build_config_game_variants(blfs_path: &String, hopper_directory: &String, config_path: &String) {
        let mut task = console_task::start(String::from("Converting Game Variants"));

        let game_variants_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from("game_variants"),
        ]);

        create_dir_all(&game_variants_folder).unwrap();

        let current_hoppers_blf_folder = build_path(vec![
            blfs_path,
            hopper_directory,
        ]);

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(&current_hoppers_blf_folder).unwrap_or_else(|err|{
            println!("{}", err);
            panic!();
        });

        let mut games_count = 0;

        for subfolder in hopper_directory_subfolders {
            if !hopper_folder_regex.is_match(&subfolder) {
                continue;
            }

            let game_variant_blfs_folder = build_path(vec![
                &current_hoppers_blf_folder,
                &subfolder,
            ]);

            if !exists(&game_variant_blfs_folder).unwrap() {
                continue;
            }

            let game_variant_files = get_files_in_folder(&game_variant_blfs_folder).unwrap_or_else(|err|{
                println!("{}", err);
                panic!();
            });

            for game_variant_file_name in game_variant_files {
                if !game_variant_regex.is_match(&game_variant_file_name) {
                    continue;
                }

                let game_variant_blf_file_path = build_path(vec![
                    &game_variant_blfs_folder,
                    &game_variant_file_name,
                ]);

                let game_variant_json_file_path = build_path(vec![
                    &game_variants_folder,
                    &game_variant_file_name.replace("_010.bin", ".json"),
                ]);

                if exists(&game_variant_json_file_path).unwrap() {
                    remove_file(&game_variant_json_file_path).unwrap()
                }

                let packed_game_variant = find_chunk_in_file::<s_blf_chunk_packed_game_variant>(&game_variant_blf_file_path).unwrap();
                let game_variant_json = serde_json::to_string_pretty(&packed_game_variant.game_variant).unwrap();
                let mut game_variant_json_file = File::create_new(game_variant_json_file_path).unwrap();
                game_variant_json_file.write_all(game_variant_json.as_bytes()).unwrap();
                games_count += 1;
            }
        }

        task.add_message(format!("Converted {games_count} game variants."));

        task.complete();
    }

    fn build_config_game_sets(blfs_path: &String, hopper_directory: &String, config_path: &String) {
        let mut task = console_task::start(String::from("Converting Game Sets..."));

        let hoppers_folder = build_path(vec![
            config_path,
            hopper_directory,
        ]);

        create_dir_all(&hoppers_folder).unwrap();

        let current_hoppers_blf_folder = build_path(vec![
            blfs_path,
            hopper_directory,
        ]);

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(&current_hoppers_blf_folder).unwrap_or_else(|err|{
            println!("{}", err);
            panic!();
        });

        let mut game_sets_count = 0;

        for subfolder in hopper_directory_subfolders {
            if !hopper_folder_regex.is_match(&subfolder) {
                continue;
            }

            let game_set_blf_path = build_path(vec![
                &current_hoppers_blf_folder,
                &subfolder,
                &String::from("game_set_006.bin"),
            ]);

            if !exists(&game_set_blf_path).unwrap() {
                task.add_warning(format!("No game set was found for hopper \"{subfolder}\""));
                continue;
            }

            let game_set = find_chunk_in_file::<s_blf_chunk_game_set>(&game_set_blf_path).unwrap();

            let output_path = build_path(vec![
                &hoppers_folder,
                &String::from("hoppers"),
                &subfolder,
                &String::from("game_set.csv"),
            ]);

            // gross
            create_dir_all(Path::new(&output_path).parent().unwrap().to_str().unwrap()).unwrap();

            let mut output_file = File::create(&output_path).unwrap();
            output_file.write_all(build_game_set_csv(&game_set).as_bytes()).unwrap();

            game_sets_count += 1;
        }

        task.add_message(format!("Converted {game_sets_count} game sets."));

        task.complete();
    }

    // Ideally, we'd separate hopper and category descriptions separately to avoid ID conflicts...
    // But foreunner doesn't seem to make this distinction, so why should I?
    fn read_hopper_description_blfs(
        hoppers_blfs_folder: &String,
        task: &mut console_task
    ) -> HashMap<String, HashMap<u16, String>>
    {
        let mut language_descriptions_map = HashMap::<String, HashMap<u16, String>>::new();

        for language_code in k_language_suffixes {
            let hopper_descriptions_path = build_path(vec![
                hoppers_blfs_folder,
                &String::from(language_code),
                &String::from("matchmaking_hopper_descriptions_003.bin")
            ]);


            if !check_file_exists(&hopper_descriptions_path) {
                task.add_warning(format!(
                    "No {} hopper descriptions are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let hopper_description_table =
                find_chunk_in_file::<s_blf_chunk_hopper_description_table>(&hopper_descriptions_path);

            if hopper_description_table.is_err() {
                task.fail(format!("Failed to read hopper descriptions file at: {hopper_descriptions_path}"));
                panic!()
            }

            let mut hoppers_description_map = HashMap::<u16, String>::new();

            hopper_description_table.unwrap().descriptions.iter().for_each(|hopper_description| {
                hoppers_description_map.insert(hopper_description.identifier, hopper_description.description.get_string());
            });

            language_descriptions_map.insert(String::from(language_code), hoppers_description_map);
        }

        language_descriptions_map
    }

    fn build_config_hoppers(hoppers_blfs_path: &String, hoppers_config_path: &String) {
        let mut task = console_task::start(String::from("Converting Hopper Configuration..."));

        let language_hopper_descriptions
            = Self::read_hopper_description_blfs(hoppers_blfs_path, &mut task);

        let hopper_configuration_blf_path = build_path(vec![
            hoppers_blfs_path,
            &String::from("matchmaking_hopper_011.bin")
        ]);

        let hopper_configuration_table = find_chunk_in_file::<s_blf_chunk_hopper_configuration_table>(&hopper_configuration_blf_path).unwrap();
        let hopper_configurations = hopper_configuration_table.get_hopper_configurations();
        let category_configurations = hopper_configuration_table.get_hopper_categories();

        // Generate active_hoppers.txt
        let active_hopper_ids = hopper_configurations.iter().map(|config|config.hopper_identifier);
        let active_hoppers_txt_path = build_path(vec![
            hoppers_config_path,
            &String::from("active_hoppers.txt"),
        ]);
        let mut active_hoppers_txt_file = File::create(active_hoppers_txt_path).unwrap();
        active_hoppers_txt_file.write_all(
            active_hopper_ids.map(|id|format!("{id:0>5}")).collect::<Vec<_>>().join("\r\n").as_bytes()
        ).unwrap();

        // Build hopper configuration json
        for hopper_configuration in hopper_configurations {
            let mut hopper_configuration_json = json_hopper_configuration {
                descriptions: HashMap::new(),
                configuration: hopper_configuration,
            };

            for language_code in k_language_suffixes {
                if language_hopper_descriptions.contains_key(language_code)
                    && language_hopper_descriptions.get(language_code).unwrap().contains_key(&hopper_configuration_json.configuration.hopper_identifier)
                {
                    hopper_configuration_json.descriptions.insert(
                        String::from(language_code),
                        language_hopper_descriptions.get(language_code).unwrap().get(&hopper_configuration_json.configuration.hopper_identifier).unwrap().clone()
                    );
                }
                else {
                    hopper_configuration_json.descriptions.insert(String::from(language_code), String::new());
                }
            }

            let hopper_configuration_json_folder = build_path(vec![
                hoppers_config_path,
                &String::from("hoppers"),
                &format!("{:0>5}", hopper_configuration_json.configuration.hopper_identifier),
            ]);
            create_dir_all(&hopper_configuration_json_folder).unwrap();

            let hopper_configuration_json_file = build_path(vec![
                &hopper_configuration_json_folder,
                &String::from("configuration.json"),
            ]);
            let mut hopper_configuration_json_file = File::create(hopper_configuration_json_file).unwrap();
            serde_json::to_writer_pretty(&mut hopper_configuration_json_file, &hopper_configuration_json).unwrap();
        }

        // Build categories json
        let mut categories_config = json_categories_configuration::default();

        for category_configuration in category_configurations {
            let mut category_configuration_and_description = category_configuration_and_descriptions {
                descriptions: HashMap::new(),
                configuration: category_configuration,
            };

            for language_code in k_language_suffixes {
                if language_hopper_descriptions.contains_key(language_code)
                    && language_hopper_descriptions.get(language_code).unwrap().contains_key(&category_configuration_and_description.configuration.category_identifier)
                {
                    category_configuration_and_description.descriptions.insert(
                        String::from(language_code),
                        language_hopper_descriptions.get(language_code).unwrap().get(&category_configuration_and_description.configuration.category_identifier).unwrap().clone()
                    );
                }
                else {
                    category_configuration_and_description.descriptions.insert(String::from(language_code), String::new());
                }
            }

            categories_config.categories.push(category_configuration_and_description);
        }


        let categories_json_file = build_path(vec![
            hoppers_config_path,
            &String::from("categories.json"),
        ]);

        let mut categories_json_file = File::create(categories_json_file).unwrap();
        serde_json::to_writer_pretty(&mut categories_json_file, &categories_config).unwrap();

        task.add_message(format!("Converted {} hopper configurations.", hopper_configuration_table.hopper_configuration_count));

        task.complete();
    }

    fn build_config_network_configuration(hoppers_blfs_path: &String, hoppers_config_path: &String) {
        // For now we just copy it as is. But we do check that it contains a netc.
        let task = console_task::start(String::from("Converting Network Configuration"));

        let network_configuration_source_path = build_path(vec![
            hoppers_blfs_path,
            &String::from("network_configuration_135.bin"),
        ]);

        let network_configuration_dest_path = build_path(vec![
            hoppers_config_path,
            &String::from("network_configuration_135.bin"),
        ]);

        // We read and rewrite to tidy any padding and the headers.
        let mut network_config = network_configuration::read(&network_configuration_source_path);
        network_config.write(&network_configuration_dest_path);

        fs::copy(network_configuration_source_path, network_configuration_dest_path).unwrap();

        task.complete();
    }

    fn build_blf_banhammer_messages(config_path: &String, hopper_directory: &String, blfs_path: &String) {
        let mut task = console_task::start(String::from("Converting Banhammer Messages"));

        let config_banhammer_messages_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from("banhammer_messages"),
        ]);

        let blf_hoppers_folder = build_path(vec![
            blfs_path,
            hopper_directory,
        ]);

        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}.txt");
            let config_file_path = format!("{config_banhammer_messages_folder}{FILE_SEPARATOR}{relative_file_path}");

            let output_file_name = "matchmaking_banhammer_messages.bin";
            let output_hopper_folder = format!("{blf_hoppers_folder}{FILE_SEPARATOR}{language_code}");
            let output_blf_path = format!("{output_hopper_folder}{FILE_SEPARATOR}{output_file_name}");

            create_dir_all(&output_hopper_folder).unwrap();

            if !check_file_exists(&config_file_path) {
                task.add_warning(format!(
                    "No {} banhammer messages are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let mut config_file = File::open(config_file_path).unwrap();
            let mut matchmaking_banhammer_messages: String = String::new();
            config_file.read_to_string(&mut matchmaking_banhammer_messages).unwrap();

            let matchmaking_banhammer_messages = matchmaking_banhammer_messages.lines().map(String::from).collect();
            let mut matchmaking_banhammer_messages = matchmaking_banhammer_messages_blf::create(matchmaking_banhammer_messages);
            matchmaking_banhammer_messages.write(&output_blf_path);
        }

        task.complete();
    }

    fn build_blf_matchmaking_tips(config_path: &String, hopper_directory: &String, blfs_path: &String) {
        let mut task = console_task::start(String::from("Converting Matchmaking Tips"));

        let config_banhammer_messages_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from("matchmaking_tips"),
        ]);

        let blf_hoppers_folder = build_path(vec![
            blfs_path,
            hopper_directory,
        ]);

        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}.txt");
            let config_file_path = format!("{config_banhammer_messages_folder}{FILE_SEPARATOR}{relative_file_path}");

            let output_file_name = "matchmaking_tips.bin";
            let output_hopper_folder = format!("{blf_hoppers_folder}{FILE_SEPARATOR}{language_code}");
            let output_blf_path = format!("{output_hopper_folder}{FILE_SEPARATOR}{output_file_name}");

            create_dir_all(&output_hopper_folder).unwrap();

            if !check_file_exists(&config_file_path) {
                task.add_warning(format!(
                    "No {} matchmaking tips are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let mut config_file = File::open(config_file_path).unwrap();
            let mut matchmaking_tips: String = String::new();
            config_file.read_to_string(&mut matchmaking_tips).unwrap();

            let matchmaking_tips = matchmaking_tips.lines().map(String::from).collect();
            let mut matchmaking_tips = matchmaking_tips_blf::create(matchmaking_tips);
            matchmaking_tips.write(&output_blf_path);
        }

        task.complete();
    }

    fn build_blf_motds(config_path: &String, hopper_directory: &String, blfs_path: &String, mythic: bool) {
        let mut task = console_task::start(
            if mythic { String::from("Converting Mythic MOTDs") }
            else { String::from("Converting MOTDs") }
        );

        let config_motds_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from(if mythic { "motd_mythic" } else { "motd" }),
        ]);

        let blf_hoppers_folder = build_path(vec![
            blfs_path,
            hopper_directory,
        ]);

        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}.txt");
            let config_file_path = format!("{config_motds_folder}{FILE_SEPARATOR}{relative_file_path}");

            let output_file_name = if mythic { "blue_motd.bin" } else { "motd.bin" };
            let output_hopper_folder = format!("{blf_hoppers_folder}{FILE_SEPARATOR}{language_code}");
            let output_blf_path = format!("{output_hopper_folder}{FILE_SEPARATOR}{output_file_name}");

            create_dir_all(&output_hopper_folder).unwrap();

            if !check_file_exists(&config_file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            let mut config_file = File::open(config_file_path).unwrap();
            let mut motd_string: String = String::new();
            config_file.read_to_string(&mut motd_string).unwrap();

            let mut motd = motd::motd::create(motd_string);
            motd.write(&output_blf_path);
        }

        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}.jpg");
            let jpeg_file_path = format!("{config_motds_folder}{FILE_SEPARATOR}{relative_file_path}");

            let output_file_name = if mythic { "blue_motd_image.jpg" } else { "motd_image.jpg" };
            let output_hopper_folder = format!("{blf_hoppers_folder}{FILE_SEPARATOR}{language_code}");

            let output_jpeg_path = format!("{output_hopper_folder}{FILE_SEPARATOR}{output_file_name}");

            create_dir_all(&output_hopper_folder).unwrap();

            if !check_file_exists(&jpeg_file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD Image is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            fs::copy(jpeg_file_path, output_jpeg_path).unwrap();
        }

        task.complete();
    }

    fn build_blf_motd_popups(config_path: &String, hopper_directory: &String, blfs_path: &String, mythic: bool) {
        let mut task = console_task::start(
            if mythic { String::from("Converting Mythic MOTD Popups") }
            else { String::from("Converting MOTD Popups") }
        );

        let config_motds_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from(if mythic { "popup_mythic" } else { "popup" }),
        ]);

        let blf_hoppers_folder = build_path(vec![
            blfs_path,
            hopper_directory,
        ]);

        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}.json");
            let config_file_path = format!("{config_motds_folder}{FILE_SEPARATOR}{relative_file_path}");

            let output_file_name = if mythic { "blue_motd_popup.bin" } else { "motd_popup.bin" };
            let output_hopper_folder = format!("{blf_hoppers_folder}{FILE_SEPARATOR}{language_code}");
            let output_blf_path = format!("{output_hopper_folder}{FILE_SEPARATOR}{output_file_name}");

            create_dir_all(&output_hopper_folder).unwrap();

            if !check_file_exists(&config_file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD Popup is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            let mut config_file = File::open(&config_file_path).unwrap();
            let mut motd_json: String = String::new();
            config_file.read_to_string(&mut motd_json).unwrap();
            let motd_popup_config: motd_popup_config = serde_json::from_str(&motd_json).unwrap();
            let mut motd_popup_blf = motd_popup_blf::create(motd_popup_config.to_chunk().unwrap());

            motd_popup_blf.write(&output_blf_path);
        }

        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}.jpg");
            let jpeg_file_path = format!("{config_motds_folder}{FILE_SEPARATOR}{relative_file_path}");

            let output_file_name = if mythic { "blue_motd_popup_image.jpg" } else { "motd_popup_image.jpg" };
            let output_hopper_folder = format!("{blf_hoppers_folder}{FILE_SEPARATOR}{language_code}");

            let output_jpeg_path = format!("{output_hopper_folder}{FILE_SEPARATOR}{output_file_name}");

            create_dir_all(&output_hopper_folder).unwrap();

            if !check_file_exists(&jpeg_file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD Image is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            fs::copy(jpeg_file_path, output_jpeg_path).unwrap();
        }

        task.complete();
    }

    fn build_blf_map_manifest(config_path: &String, hopper_directory: &String, blfs_path: &String)
    {
        let mut task = console_task::start(String::from("Building Map Manifest"));

        let rsa_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from("rsa_signatures")
        ]);

        let rsa_files = get_files_in_folder(&rsa_folder).unwrap_or_else(|err|{
            task.add_error(err);
            task.complete();
            panic!();
        });

        if rsa_files.len() < 1 {
            task.fail(String::from("No RSA signatures found."));
            return;
        }

        let mut map_manifest = s_blf_chunk_map_manifest::default();

        for rsa_file_name in rsa_files {
            let rsa_file_path = build_path(vec![
                &rsa_folder,
                &rsa_file_name,
            ]);
            let rsa_file = File::open(&rsa_file_path);
            if rsa_file.is_err() {
                task.add_error(format!("Failed to open RSA signature: {rsa_file_path}"));
                task.complete();
                return;
            }
            let mut rsa_file = rsa_file.unwrap();
            let mut rsa_signature = Vec::<u8>::with_capacity(0x100);
            rsa_file.read_to_end(&mut rsa_signature).unwrap();

            let result = map_manifest.add_rsa_signature(rsa_signature.as_slice());
            if result.is_err() {
                task.add_error(format!("Failed to add RSA signature {rsa_file_name} to manifest: {}", result.unwrap_err()));
                task.complete();
                return;
            }
        }

        let output_file_path = build_path(vec![
            blfs_path,
            hopper_directory,
            &String::from("rsa_manifest.bin"),
        ]);

        let mut rsa_manifest = rsa_manifest::rsa_manifest::create(&map_manifest);
        rsa_manifest.write(&output_file_path);

        task.add_message(format!("Added {} RSA signatures.", map_manifest.get_rsa_signatures().len()));

        task.complete();
    }

    fn read_active_hopper_configuration(hoppers_config_path: &String) -> Vec<String> {
        let task = console_task::start(String::from("Reading Active Hoppers..."));

        let active_hoppers_folders = read_active_hoppers(hoppers_config_path).unwrap_or_else(|err| {
            task.fail(err);
            panic!();
        });

        task.complete();

        active_hoppers_folders
    }

    fn read_game_set_configuration(hoppers_config_path: &String, active_hopper_folders: &Vec<String>) -> HashMap<u16, game_set>
    {
        let task = console_task::start(String::from("Reading Game Sets..."));

        let mut game_sets = HashMap::<u16, game_set>::new();

        let hopper_tables_config_path = build_path(vec![
            &hoppers_config_path,
            &String::from("hoppers")
        ]);

        for subfolder in active_hopper_folders {
            let hopper_id= config_hopper_folder_identifier_regex.captures(subfolder);
            if !&hopper_id.is_some() {
                continue;
            }
            let hopper_id = hopper_id.unwrap();
            if !hopper_id.get(0).is_some() {
                continue;
            }
            let hopper_id = hopper_id.get(0).unwrap().as_str();
            let hopper_id = u16::from_str(hopper_id).unwrap();

            let game_set_csv_path = build_path(vec![
                &hopper_tables_config_path,
                &subfolder.to_string(),
                &String::from("game_set.csv"),
            ]);

            if !exists(&game_set_csv_path).unwrap() {
                task.fail(format!("No game set was found for hopper \"{subfolder}\""));
                panic!();
            }

            let game_set = game_set::read(game_set_csv_path).unwrap_or_else(|err| {
                task.fail(err);
                panic!();
            });

            game_sets.insert(hopper_id, game_set);
        }

        task.complete();

        game_sets
    }

    fn build_blf_game_variants(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        build_temp_dir: &String,
        game_sets: &HashMap<u16, game_set>,
        variant_hashes: &mut HashMap<String, s_network_http_request_hash>
    )
    {
        let mut task = console_task::start(String::from("Building Game Variants"));

        let game_variants_config_path = build_path(vec![
            &hoppers_config_path,
            &String::from("game_variants")
        ]);

        let game_variants_temp_build_path = build_path(vec![
            build_temp_dir,
            &String::from("game_variants")
        ]);

        let rt = runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .build()
            .unwrap();

        create_dir_all(&game_variants_temp_build_path).unwrap();

        let game_variants_to_convert: Vec<String> = game_sets.iter().flat_map(|(_, game_set)|
            game_set.entries.iter().map(|entry|entry.game_variant_file_name.clone()).collect::<Vec<String>>()
        ).collect();

        let game_variants_to_convert: HashSet<String> = HashSet::from_iter(game_variants_to_convert.iter().cloned());

        for game_variant_file_name in &game_variants_to_convert {
            let game_variant_json_path = build_path(vec![
                &game_variants_config_path,
                &format!("{game_variant_file_name}.json"),
            ]);

            let game_variant_blf_path = build_path(vec![
                &game_variants_temp_build_path,
                &format!("{game_variant_file_name}_010.bin"),
            ]);

            if !exists(&game_variant_json_path).unwrap() {
                task.fail(format!("Game variant \"{game_variant_file_name}\" could not be found."))
            }

            let mut file = File::open(&game_variant_json_path).unwrap();
            let game_variant_json: c_game_variant = serde_json::from_reader(&mut file).unwrap();
            let mut game_variant_blf_file = game_variant::create(game_variant_json);
            game_variant_blf_file.write(&game_variant_blf_path);

            variant_hashes.insert(game_variant_file_name.clone(), get_blf_file_hash(game_variant_blf_path));
        }

        task.add_message(format!("Built {} variants.", game_variants_to_convert.len()));

        task.complete();
    }

    pub fn get_scenario_rsa_crc32s(hoppers_config_path: &String) -> HashMap<u32, u32> {
        let mut result = HashMap::<u32, u32>::new();

        let rsa_folder = build_path(vec![
            hoppers_config_path,
            &String::from("rsa_signatures")
        ]);

        if !exists(&rsa_folder).unwrap() {
            return result;
        }

        let rsa_files = get_files_in_folder(&rsa_folder).unwrap_or_else(|err|{
            panic!();
        });

        for rsa_file_name in rsa_files {
            let rsa_file_path = build_path(vec![
                &rsa_folder,
                &rsa_file_name,
            ]);
            let rsa_file = File::open(&rsa_file_path);
            if rsa_file.is_err() {
                continue;
            }
            let mut rsa_file = rsa_file.unwrap();
            let mut rsa_signature = Vec::<u8>::with_capacity(0x100);
            rsa_file.read_to_end(&mut rsa_signature).unwrap();

            let map_id = config_rsa_signature_file_map_id_regex.captures(rsa_file_name.as_str()).unwrap();
            let map_id = map_id.get(0).unwrap();
            let map_id = u32::from_str(map_id.as_str()).unwrap();
            let crc32 = crc32(0xFFFFFFFF, &rsa_signature);

            result.insert(map_id, crc32);
        }

        result
    }

    fn build_blf_map_variants(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        build_temp_dir: &String,
        game_sets: &HashMap<u16, game_set>,
        variant_hashes: &mut HashMap<String, s_network_http_request_hash>,
        variant_map_ids: &mut HashMap<String, u32>
    )
    {
        let task = Arc::new(Mutex::new(console_task::start(String::from("Building Map Variants"))));

        let scenario_crc32s = Arc::new(Self::get_scenario_rsa_crc32s(&hoppers_config_path));

        let map_variants_config_path = build_path(vec![
            hoppers_config_path,
            &String::from("map_variants"),
        ]);

        let map_variants_temp_build_path = build_path(vec![
            build_temp_dir,
            &String::from("map_variants"),
        ]);

        let rt = runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .build()
            .unwrap();

        create_dir_all(&map_variants_temp_build_path).unwrap();

        let map_variants_to_convert: Vec<String> = game_sets.iter().flat_map(|(_, game_set)|
            game_set.entries.iter().map(|entry| entry.map_variant_file_name.clone()).collect::<Vec<String>>()
        ).collect();
        let map_variants_to_convert: HashSet<String> = HashSet::from_iter(map_variants_to_convert.iter().cloned());

        let mut json_queue: Vec<(String, String)> = Vec::new();
        for map_variant in map_variants_to_convert {
            let map_variant_json_path = build_path(vec![
                &map_variants_config_path,
                &format!("{map_variant}.json"),
            ]);

            if !Path::new(&map_variant_json_path).exists() {
                eprintln!("Map variant \"{}\" could not be found.", map_variant);
                continue;
            }

            let mut file = File::open(&map_variant_json_path).unwrap();
            let mut map_variant_json = String::new();
            file.read_to_string(&mut map_variant_json).unwrap();

            json_queue.push((map_variant, map_variant_json));
        }

        let json_queue = Arc::new(Mutex::new(VecDeque::from(json_queue)));

        let shared_variant_hashes = Arc::new(Mutex::new(HashMap::new()));
        let shared_variant_map_ids = Arc::new(Mutex::new(HashMap::new()));

        let cpu_cores = num_cpus::get();
        rt.block_on(async {
            let mut thread_handles = Vec::<JoinHandle<()>>::with_capacity(cpu_cores);

            for n in 0..cpu_cores {
                let shared_variant_hashes = Arc::clone(&shared_variant_hashes);
                let shared_variant_map_ids = Arc::clone(&shared_variant_map_ids);
                let map_variants_config_path = map_variants_config_path.clone();
                let map_variants_temp_build_path = map_variants_temp_build_path.clone();
                let scenario_crc32s = Arc::clone(&scenario_crc32s);
                let task = Arc::clone(&task);
                let json_queue = Arc::clone(&json_queue);

                thread_handles.push(rt.spawn(async move {
                    loop {
                        let mut json_queue = json_queue.lock().await;

                        if let Some((map_variant_file_name, json)) = json_queue.pop_front() {
                            let remaining = json_queue.len();
                            drop(json_queue);

                            // println!("[MAPS] Thread {n} got {map_variant_file_name} ({remaining} remaining)");

                            let map_variant_blf_path = build_path(vec![
                                &map_variants_temp_build_path,
                                &format!("{map_variant_file_name}_012.bin"),
                            ]);

                            let mut map_variant_json: c_map_variant = serde_json::from_str(&json).unwrap();

                            // Check the scenario crc
                            let expected_scenario_crc = scenario_crc32s.get(&map_variant_json.m_map_id);
                            if expected_scenario_crc.is_none() {
                                let mut task = task.lock().await;
                                task.add_error(format!("Map Variant {map_variant_file_name} could not be validated due to missing RSA signature!"))
                            }
                            else {
                                let expected_scenario_crc = expected_scenario_crc.unwrap();
                                if expected_scenario_crc != &map_variant_json.m_map_variant_checksum {
                                    let mut task = task.lock().await;
                                    task.add_error(format!("Map Variant {map_variant_file_name} has a bad checksum and may not load properly! (expected {:08X})", expected_scenario_crc));
                                    map_variant_json.m_map_variant_checksum = expected_scenario_crc.clone();
                                }
                            }

                            let mut map_ids = shared_variant_map_ids.lock().await;
                            map_ids.insert(map_variant_file_name.clone(), map_variant_json.m_map_id);

                            let mut map_variant_blf_file = map_variant::create(map_variant_json);
                            map_variant_blf_file.write(&map_variant_blf_path);

                            let hash = get_blf_file_hash(map_variant_blf_path);
                            let mut hashes = shared_variant_hashes.lock().await;
                            hashes.insert(map_variant_file_name.clone(), hash);
                        } else {
                            break;
                        }
                    }
                }));
            }

            for thread_handle in thread_handles {
                thread_handle.await.unwrap();
            }

            let final_hashes = shared_variant_hashes.lock().await;
            variant_hashes.extend(final_hashes.clone());

            let final_map_ids = shared_variant_map_ids.lock().await;
            variant_map_ids.extend(final_map_ids.clone());

            let mut task = task.lock().await;
            task.add_message(format!("Built {} variants.", variant_hashes.len()));
            task.complete();
        });
    }

    fn build_blf_game_sets(
        hoppers_blf_path: &String,
        active_game_sets: HashMap<u16, game_set>,
        game_variant_hashes: &HashMap<String, s_network_http_request_hash>,
        map_variant_hashes: &HashMap<String, s_network_http_request_hash>,
        map_variant_map_ids: &HashMap<String, u32>,
        build_temp_dir_path: &String,
    )
    {
        let task = console_task::start(String::from("Building Game Sets"));

        for (hopper_id, game_set) in active_game_sets {
            let hopper_folder_path = build_path(vec![
                hoppers_blf_path,
                &format!("{hopper_id:0>5}"),
            ]);

            let hopper_folder_map_variants_path = build_path(vec![
                &hopper_folder_path,
                &"map_variants".to_string(),
            ]);

            let game_variants_temp_build_path = build_path(vec![
                build_temp_dir_path,
                &String::from("game_variants")
            ]);

            let map_variants_temp_build_path = build_path(vec![
                build_temp_dir_path,
                &String::from("map_variants")
            ]);

            create_dir_all(&hopper_folder_map_variants_path).unwrap();

            let mut game_sets_blf_chunk = s_blf_chunk_game_set::default();

            let copied_maps = HashSet::<String>::new();
            let copied_games = HashSet::<String>::new();

            for game_set_row in game_set.entries {
                let map_id = map_variant_map_ids.get(&game_set_row.map_variant_file_name)
                    .expect("No map ID found for map in game set!")
                    .clone();

                let map_hash = map_variant_hashes.get(&game_set_row.map_variant_file_name)
                    .expect("No map hash found for map in game set!")
                    .clone();

                let game_hash = game_variant_hashes.get(&game_set_row.game_variant_file_name)
                    .expect("No game hash found for map in game set!")
                    .clone();

                // Create the game set entry...
                game_sets_blf_chunk.add_entry(s_blf_chunk_game_set_entry {
                    map_id,
                    map_variant_file_name: StaticString::from_string(&game_set_row.map_variant_file_name).unwrap(),
                    game_variant_file_name: StaticString::from_string(&game_set_row.game_variant_file_name).unwrap(),
                    map_variant_file_hash: map_hash,
                    game_variant_file_hash: game_hash,
                    weight: game_set_row.weight,
                    optional: game_set_row.optional,
                    skip_after_veto: game_set_row.skip_after_veto,
                    minimum_player_count: game_set_row.minimum_player_count,
                }).unwrap();

                // Copy the game and map variants over...
                if !copied_games.contains(&game_set_row.game_variant_file_name) {
                    let game_variant_file_name = format!("{}_010.bin", game_set_row.game_variant_file_name);
                    fs::copy(
                        build_path(vec![
                            &game_variants_temp_build_path,
                            &game_variant_file_name,
                        ]),
                        build_path(vec![
                            &hopper_folder_path,
                            &game_variant_file_name,
                        ])
                    ).unwrap();
                }

                if !copied_maps.contains(&game_set_row.map_variant_file_name) {
                    let map_variant_file_name = format!("{}_012.bin", game_set_row.map_variant_file_name);
                    fs::copy(
                        build_path(vec![
                            &map_variants_temp_build_path,
                            &map_variant_file_name,
                        ]),
                        build_path(vec![
                            &hopper_folder_map_variants_path,
                            &map_variant_file_name,
                        ])
                    ).unwrap();
                }
            }

            // Write the game set file
            let mut game_set_blf_file = blf_files::game_set::game_set::create(game_sets_blf_chunk);
            game_set_blf_file.write(&build_path(vec![
                &hopper_folder_path,
                &String::from(blf_files::game_set::k_game_set_file_name),
            ]))
        }

        task.complete();
    }
}