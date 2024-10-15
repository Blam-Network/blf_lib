mod matchmaking_banhammer_messages;
mod motd;
mod rsa_manifest;

use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use crate::io::{build_path, get_directories_in_folder, get_files_in_folder, FILE_SEPARATOR};
use crate::title_converter;
use crate::title_storage::{check_file_exists, TitleConverter};
use inline_colorization::*;
use blf_lib::blam::cseries::language::{get_language_string, k_language_suffix_chinese_traditional, k_language_suffix_english, k_language_suffix_french, k_language_suffix_german, k_language_suffix_italian, k_language_suffix_japanese, k_language_suffix_korean, k_language_suffix_mexican, k_language_suffix_portuguese, k_language_suffix_spanish};
use blf_lib::blf::BlfFile;
use blf_lib::blf::chunks::find_chunk_in_file;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_banhammer_messages, s_blf_chunk_map_manifest, s_blf_chunk_matchmaking_tips, s_blf_chunk_message_of_the_day};
use crate::console::{console_task};

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

        let hopper_directories = get_directories_in_folder(&config_path);

        for hopper_directory in hopper_directories {
            if hopper_directory.len() > HOPPER_DIRECTORY_NAME_MAX_LENGTH {
                println!("{color_bright_white}{bg_red}Skipping \"{hopper_directory}\" as it's name is too long. ({HOPPER_DIRECTORY_NAME_MAX_LENGTH} characters MAX){style_reset}");
                continue;
            }

            println!("{style_bold}Converting {color_bright_white}{}{style_reset}...", hopper_directory);
            Self::build_blf_motds(config_path, &hopper_directory, blfs_path, false);
            Self::build_blf_motds(config_path, &hopper_directory, blfs_path, true);
            Self::build_blf_map_manifest(config_path, &hopper_directory, blfs_path);
        }
    }

    fn build_config(&mut self, blfs_path: &String, config_path: &String) {
        println!("{style_bold}Writing Title Storage config to {config_path} {style_reset}");

        let hopper_directories = get_directories_in_folder(&blfs_path);

        for hopper_directory in hopper_directories {
            if hopper_directory.len() > HOPPER_DIRECTORY_NAME_MAX_LENGTH {
                println!("{color_bright_white}{bg_red}Skipping \"{hopper_directory}\" as it's name is too long. ({HOPPER_DIRECTORY_NAME_MAX_LENGTH} characters MAX){style_reset}");
                continue;
            }

            println!("{style_bold}Converting {color_bright_white}{}{style_reset}...", hopper_directory);
            Self::build_config_banhammer_messages(blfs_path, &hopper_directory, config_path);
            Self::build_config_motds(blfs_path, &hopper_directory, config_path, false);
            Self::build_config_motds(blfs_path, &hopper_directory, config_path, true);
            Self::build_config_matchmaking_tips(blfs_path, &hopper_directory, config_path);

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
            &String::from("tips"),
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

    fn build_blf_map_manifest(config_path: &String, hopper_directory: &String, blfs_path: &String) {
        let mut task = console_task::start(String::from("Building Map Manifest"));

        let rsa_folder = build_path(vec![
            config_path,
            hopper_directory,
            &String::from("rsa_signatures")
        ]);

        let rsa_files = get_files_in_folder(&rsa_folder);

        if rsa_files.len() < 1 {
            task.add_error(String::from("No RSA signatures found."));
            task.complete();
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

        let mut rsa_manifest = rsa_manifest::rsa_manifest::create(map_manifest);
        rsa_manifest.write(&output_file_path);

        task.complete();
    }
}