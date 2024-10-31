use std::collections::{HashMap, HashSet};
use std::fs::{create_dir_all, exists, remove_file, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use crate::io::{build_path, get_directories_in_folder, get_files_in_folder, FILE_SEPARATOR};
use crate::title_converter;
use crate::title_storage::{check_file_exists, TitleConverter};
use inline_colorization::*;
use lazy_static::lazy_static;
use blf_lib::blam::common::cseries::language::{get_language_string, k_language_suffix_chinese_traditional, k_language_suffix_english, k_language_suffix_french, k_language_suffix_german, k_language_suffix_italian, k_language_suffix_japanese, k_language_suffix_korean, k_language_suffix_mexican, k_language_suffix_portuguese, k_language_suffix_spanish};
use blf_lib::blf::{get_blf_file_hash, BlfFile};
use blf_lib::blf::chunks::find_chunk_in_file;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_banhammer_messages, s_blf_chunk_game_set, s_blf_chunk_hopper_description_table, s_blf_chunk_map_manifest, s_blf_chunk_matchmaking_tips, s_blf_chunk_message_of_the_day, s_blf_chunk_message_of_the_day_popup, s_blf_chunk_packed_game_variant, s_blf_chunk_packed_map_variant};
use crate::console::console_task;
use crate::title_storage::halo3::release::blf_files::{motd, rsa_manifest};
use crate::title_storage::halo3::release::config_files::motd_popup::motd_popup as motd_popup_config;
use crate::title_storage::halo3::release::blf_files::motd_popup::motd_popup as motd_popup_blf;
use crate::title_storage::halo3::release::blf_files::matchmaking_banhammer_messages::{matchmaking_banhammer_messages as matchmaking_banhammer_messages_blf};
use crate::title_storage::halo3::release::blf_files::matchmaking_tips::matchmaking_tips as matchmaking_tips_blf;
use regex::Regex;
use tempdir::TempDir;
use tokio::runtime;
use tokio::sync::{mpsc, Mutex};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_hopper_configuration_table;
use crate::title_storage::halo3::release::blf_files::game_variant::game_variant;
use crate::title_storage::halo3::release::blf_files::map_variant::map_variant;
use crate::title_storage::halo3::release::config_files::game_set::{build_game_set_csv, game_set};

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

        let build_temp_dir = TempDir::new("blf_cli").unwrap();
        let build_temp_dir_path = String::from(build_temp_dir.path().to_str().unwrap());

        println!("Using temp directory: {build_temp_dir_path}");

        let hopper_directories = get_directories_in_folder(&config_path).unwrap_or_else(|err|{
            println!("{}", err);
            panic!()
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
            Self::build_blf_banhammer_messages(config_path, &hopper_directory, blfs_path);
            Self::build_blf_matchmaking_tips(config_path, &hopper_directory, blfs_path);
            Self::build_blf_motds(config_path, &hopper_directory, blfs_path, false);
            Self::build_blf_motds(config_path, &hopper_directory, blfs_path, true);
            Self::build_blf_motd_popups(config_path, &hopper_directory, blfs_path, false);
            Self::build_blf_motd_popups(config_path, &hopper_directory, blfs_path, true);
            Self::build_blf_map_manifest(config_path, &hopper_directory, blfs_path);

            let game_sets = Self::read_game_set_configuration(&hopper_config_path, &String::new());
            let mut game_variant_hashes = HashMap::<String, s_network_http_request_hash>::new();
            let mut map_variant_hashes = HashMap::<String, s_network_http_request_hash>::new();

            Self::build_blf_game_variants(&hopper_config_path, &hopper_blfs_path, &build_temp_dir_path, &game_sets, &mut game_variant_hashes);
            Self::build_blf_map_variants(&hopper_config_path, &hopper_blfs_path, &build_temp_dir_path, &game_sets, &mut map_variant_hashes);

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
    static ref hopper_config_folder_identifier_regex: Regex = Regex::new(r"^[0-9]{1,5}").unwrap();
    static ref map_variant_regex: Regex = Regex::new(r"_012.bin$").unwrap();
    static ref game_variant_regex: Regex = Regex::new(r"_010.bin$").unwrap();
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
    ) -> HashMap<String, HashMap<u16, String>> {
        let mut task = console_task::start(String::from("Reading Hopper Descriptions..."));

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
            = Self::read_hopper_description_blfs(hoppers_blfs_path);

        let hopper_configuration_blf_path = build_path(vec![
            hoppers_blfs_path,
            &String::from("matchmaking_hopper_011.bin")
        ]);

        let hopper_configuration_table = find_chunk_in_file::<s_blf_chunk_hopper_configuration_table>(&hopper_configuration_blf_path).unwrap();
        let hopper_configurations = hopper_configuration_table.get_hopper_configurations();

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


        task.add_message(format!("Converted {} hopper configurations.", hopper_configuration_table.hopper_configuration_count));

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

            std::fs::copy(jpeg_file_path, output_jpeg_path).unwrap();
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

            std::fs::copy(jpeg_file_path, output_jpeg_path).unwrap();
        }

        task.complete();
    }

    fn build_blf_map_manifest(config_path: &String, hopper_directory: &String, blfs_path: &String) {
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

    fn read_game_set_configuration(hoppers_config_path: &String, active_hoppers: &String) -> Vec<game_set> {
        let mut task = console_task::start(String::from("Reading Game Sets..."));

        let mut game_sets = Vec::<game_set>::new();
        let hopper_tables_config_path = build_path(vec![
            &hoppers_config_path,
            &String::from("hoppers")
        ]);

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(&hopper_tables_config_path).unwrap_or_else(|err|{
            println!("{}", err);
            panic!();
        });

        for subfolder in hopper_directory_subfolders {
            if !hopper_folder_regex.is_match(&subfolder) {
                continue;
            }

            let game_set_csv_path = build_path(vec![
                &hopper_tables_config_path,
                &subfolder,
                &String::from("game_set.csv"),
            ]);

            if !exists(&game_set_csv_path).unwrap() {
                task.add_error(format!("No game set was found for hopper \"{subfolder}\""));
                // panic!();
            }

            let game_set = game_set::read(game_set_csv_path).unwrap_or_else(|err| {
                task.fail(err);
                panic!();
            });

            game_sets.push(game_set);
        }

        task.complete();

        game_sets
    }

    fn build_blf_game_variants(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        build_temp_dir: &String,
        game_sets: &Vec<game_set>,
        variant_hashes: &mut HashMap<String, s_network_http_request_hash>
    ) {
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

        let game_variants_to_convert: Vec<String> = game_sets.iter().flat_map(|game_set|
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

    fn build_blf_map_variants(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        build_temp_dir: &String,
        game_sets: &Vec<game_set>,
        variant_hashes: &mut HashMap<String, s_network_http_request_hash>,
    ) {
        let mut task = console_task::start(String::from("Building Map Variants"));

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

        let map_variants_to_convert: Vec<String> = game_sets.iter().flat_map(|game_set|
            game_set.entries.iter().map(|entry| entry.map_variant_file_name.clone()).collect::<Vec<String>>()
        ).collect();

        let map_variants_to_convert: HashSet<String> = HashSet::from_iter(map_variants_to_convert.iter().cloned());

        let shared_variant_hashes = Arc::new(Mutex::new(HashMap::new()));
        let (tx, rx) = mpsc::channel(100);

        let cpu_cores = num_cpus::get();
        rt.block_on(async {
            let rx = Arc::new(Mutex::new(rx));

            for _ in 0..cpu_cores {
                let rx = Arc::clone(&rx);
                let shared_variant_hashes = Arc::clone(&shared_variant_hashes);
                let map_variants_config_path = map_variants_config_path.clone();
                let map_variants_temp_build_path = map_variants_temp_build_path.clone();

                rt.spawn(async move {
                    loop {
                        let map_variant_file_name = {
                            let mut rx = rx.lock().await;
                            rx.recv().await
                        };

                        match map_variant_file_name {
                            Some(map_variant_file_name) => {
                                let map_variant_json_path = build_path(vec![
                                    &map_variants_config_path,
                                    &format!("{map_variant_file_name}.json"),
                                ]);

                                let map_variant_blf_path = build_path(vec![
                                    &map_variants_temp_build_path,
                                    &format!("{map_variant_file_name}_012.bin"),
                                ]);

                                if !Path::new(&map_variant_json_path).exists() {
                                    eprintln!("Map variant \"{}\" could not be found.", map_variant_file_name);
                                    continue;
                                }

                                let mut file = File::open(&map_variant_json_path).unwrap();
                                let map_variant_json: c_map_variant = serde_json::from_reader(&mut file).unwrap();
                                let mut map_variant_blf_file = map_variant::create(map_variant_json);
                                map_variant_blf_file.write(&map_variant_blf_path);

                                let hash = get_blf_file_hash(map_variant_blf_path);
                                let mut hashes = shared_variant_hashes.lock().await;
                                hashes.insert(map_variant_file_name, hash);
                            }
                            None => break,
                        }
                    }
                });
            }

            for map_variant_file_name in map_variants_to_convert {
                tx.send(map_variant_file_name).await.unwrap();
            }

            let final_hashes = shared_variant_hashes.lock().await;
            variant_hashes.extend(final_hashes.clone());
        });

        task.add_message(format!("Built {} variants.", variant_hashes.len()));
        task.add_error("Checksums are not yet generated for map variants.".to_string());
        task.complete();
    }
}