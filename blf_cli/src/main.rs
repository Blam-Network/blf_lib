#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{command, Parser};
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_compressed_data, s_blf_chunk_content_header, s_blf_chunk_end_of_file, s_blf_chunk_screenshot_camera, s_blf_chunk_screenshot_data, s_blf_chunk_start_of_file};
use blf_lib::blf_file;
use blf_lib::io::write_json_file;
use blf_lib::types::byte_order_mark::byte_order_mark;
use crate::commands::Commands;
use crate::commands::Commands::{ConvertH3MCCMapVariants, TitleStorage};
use crate::commands::convert_halo3mcc_map_variants::convert_halo3mcc_map_variants;
use crate::commands::import_rsa_signatures::import_rsa_signatures;
use crate::commands::import_variant::import_variant;
use crate::commands::export_variant::export_variant;
use crate::commands::title_storage::TitleStorageSubcommands;

mod title_storage;
mod io;
mod console;
mod commands;
mod result;

#[derive(Debug, Parser)]
#[command(name = "blf_cli")]
#[command(about = "blam! engine file editor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

blf_file!(
    pub struct screenshot_file {
        pub _blf: s_blf_chunk_start_of_file,
        pub chdr: s_blf_chunk_content_header,
        pub scnc: s_blf_chunk_screenshot_camera,
        pub _cmp_scnd: s_blf_chunk_compressed_data<s_blf_chunk_screenshot_data>,
        pub _eof: s_blf_chunk_end_of_file
    }
);

blf_file!(
    pub struct decompressed_screenshot {
        pub _blf: s_blf_chunk_start_of_file,
        pub chdr: s_blf_chunk_content_header,
        pub scnc: s_blf_chunk_screenshot_camera,
        pub scnd: s_blf_chunk_screenshot_data,
        pub _eof: s_blf_chunk_end_of_file
    }
);

fn main() {
    // let bytes = std::fs::read("img_in.jpg").unwrap();
    let bytes = std::fs::read("C:\\Users\\stell\\Downloads\\badass.jpg").unwrap();


    let mut old_screenshot = screenshot_file::read_file(&String::from("C:\\Users\\stell\\Downloads\\anime.bin")).unwrap();

    old_screenshot.chdr.metadata.name.set_string(&String::from("BADASS"));
    old_screenshot.chdr.metadata.description.set_string(&String::from("Frieren is BADASS"));
    old_screenshot.chdr.metadata.author.set_string(&String::from("kanokoga"));
    // let mut new_screenshot = screenshot_file::read_file(&String::from("C:\\Users\\stell\\Downloads\\anime_new.bin")).unwrap();
    // std::fs::write("C:\\Users\\stell\\Downloads\\anime_new.jpg", &old_screenshot._cmp_scnd.get_chunk().jpeg_data);


    // let mut decomp_new_screenshot = decompressed_screenshot {
    //     _blf: new_screenshot._blf,
    //     chdr: new_screenshot.chdr,
    //     scnc: new_screenshot.scnc,
    //     scnd: new_screenshot._cmp_scnd.chunk,
    //     _eof: new_screenshot._eof,
    // };
    //
    // decomp_old_screenshot.write_file("C:\\Users\\stell\\Downloads\\decomp_anime.bin");
    // decomp_new_screenshot.write_file("C:\\Users\\stell\\Downloads\\decomp_anime_new.bin");

    old_screenshot._cmp_scnd.chunk.jpeg_data = bytes;
    old_screenshot.scnc.jpeg_data_length = old_screenshot._cmp_scnd.chunk.jpeg_data.len() as u32;

    old_screenshot.write_file("C:\\Users\\stell\\Downloads\\anime_new.bin");

    // let mut decomp_new_screenshot = decompressed_screenshot {
    //     _blf: old_screenshot._blf,
    //     chdr: old_screenshot.chdr,
    //     scnc: old_screenshot.scnc,
    //     scnd: old_screenshot._cmp_scnd.chunk,
    //     _eof: old_screenshot._eof,
    // };
    //
    // decomp_new_screenshot.write_file("C:\\Users\\stell\\Downloads\\decomp_anime_new.bin");



    // debug_log!("{}", serde_json::to_string_pretty(&old_screenshot.scnc).unwrap());
    //
    //
    // let mut screenshot = screenshot_file {
    //     _blf: s_blf_chunk_start_of_file::new("halo3 screenshot", byte_order_mark::default()),
    //     chdr: old_screenshot.chdr,
    //     scnc: old_screenshot.scnc,
    //     _cmp_scnd: s_blf_chunk_compressed_data::create(s_blf_chunk_screenshot_data {
    //         jpeg_data: bytes
    //     }),
    //     _eof: s_blf_chunk_end_of_file::default()
    // };
    //
    // screenshot.write_file("new_screenshot.bin");


    // std::fs::write("img_new.jpg", &old_screenshot._cmp_scnd.get_chunk().jpeg_data);


    return;


    let args = Cli::parse();

    match args.command {
        TitleStorage(title_storage_command) => match title_storage_command.command {
            TitleStorageSubcommands::Build { config_input_path, blf_output_path, title, version } => {
                let mut title_converter =
                    title_storage::get_title_converter(title, version)
                        .expect("No title converter was found for the provided title and version.");

                title_converter.build_blfs(
                    &config_input_path,
                    &blf_output_path
                );
            },
            TitleStorageSubcommands::BuildConfig { blf_input_path, config_output_path, title, version } => {
                let mut title_converter =
                    title_storage::get_title_converter(title, version)
                        .expect("No title converter was found for the provided title and version.");

                title_converter.build_config(
                    &blf_input_path,
                    &config_output_path
                );
            },
            TitleStorageSubcommands::ImportRsaSignatures { hoppers_config_path, halo_maps_folder, title, version } => {
                import_rsa_signatures(hoppers_config_path, halo_maps_folder, title, version);
            },
            TitleStorageSubcommands::ImportVariant { hoppers_config_path, variant_path, title, version } => {
                import_variant(hoppers_config_path, variant_path, title, version);
            },
            TitleStorageSubcommands::ExportVariant { variant_json_path, destination_path, title, version } => {
                export_variant(variant_json_path, destination_path, title, version);
            }
        },
        ConvertH3MCCMapVariants { mcc_maps_folder, converted_maps_folder} => {
            convert_halo3mcc_map_variants(mcc_maps_folder, converted_maps_folder);
        }
    }
}