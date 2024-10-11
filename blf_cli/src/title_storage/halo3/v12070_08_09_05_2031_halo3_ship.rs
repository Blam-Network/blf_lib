mod matchmaking_banhammer_messages;

use std::fs::{create_dir_all, File};
use std::io::Write;
use crate::io::{build_path, get_directories_in_folder, FILE_SEPARATOR};
use crate::title_converter;
use crate::title_storage::{check_file_exists, fail_step, log_warning, TitleConverter};
use inline_colorization::*;
use blf_lib::blam::cseries::language::{get_language_string, k_language_suffixes};
use blf_lib::blf::BlfFile;
use blf_lib::blf::chunks::find_chunk_in_file;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_banhammer_messages, s_blf_chunk_message_of_the_day};

title_converter! (
    #[Title("Halo 3")]
    #[Build("12070.08.09.05.2031.halo3_ship")]
    pub struct v12070_08_09_05_2031_halo3_ship {}
);

const HOPPER_DIRECTORY_NAME_MAX_LENGTH: usize = 64;

impl TitleConverter for v12070_08_09_05_2031_halo3_ship {
    fn build_blfs(&mut self, config_path: &String, blfs_path: &String) {
        todo!()
    }

    fn build_config(&mut self, blfs_path: &String, config_path: &String) {
        println!("{style_bold}Writing Title Storage config to {config_path} {style_reset}");

        let hopper_directories = get_directories_in_folder(&blfs_path);

        for hopper_directory in hopper_directories {
            if hopper_directory.len() > HOPPER_DIRECTORY_NAME_MAX_LENGTH {
                println!("{color_bright_white}{bg_red}Skipping \"{hopper_directory}\" as it's name is too long. ({HOPPER_DIRECTORY_NAME_MAX_LENGTH} characters MAX){style_reset}");
                continue;
            }

            println!("Converting {color_bright_white}{}{style_reset}...", hopper_directory);
            Self::build_config_banhammer_messages(blfs_path, &hopper_directory, config_path);
            Self::build_config_motds(blfs_path, &hopper_directory, config_path, false);
            Self::build_config_motds(blfs_path, &hopper_directory, config_path, true);

        }
    }
}

impl v12070_08_09_05_2031_halo3_ship {
    fn build_config_banhammer_messages(blfs_path: &String, hopper_directory: &String, config_path: &String) {
        println!("● Converting banhammer messages... ");

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
                log_warning(format!(
                    "No {} banhammer messages are present.",
                    get_language_string(language_code),
                ), 1);

                continue;
            }

            let banhammer_messages =
                find_chunk_in_file::<s_blf_chunk_banhammer_messages>(&file_path);

            if banhammer_messages.is_err() {
                fail_step(file_path)
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

        println!("{color_green}Done{style_reset}.");
    }

    fn build_config_motds(blfs_path: &String, hopper_directory: &String, config_path: &String, mythic: bool) {
        println!("● Converting {}MOTDs... ", if mythic { "Mythic " } else { "" });

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
                log_warning(format!(
                    "No {} {}MOTD is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ), 1);

                continue;
            }

            let mut motd =
                find_chunk_in_file::<s_blf_chunk_message_of_the_day>(&file_path);

            if motd.is_err() {
                fail_step(file_path)
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
                log_warning(format!(
                    "No {} {}MOTD image is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ), 1);

                continue;
            }

            std::fs::copy(file_path, output_path).unwrap();
        }

        println!("{color_green}Done{style_reset}.");
    }
}