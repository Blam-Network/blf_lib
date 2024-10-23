#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::{remove_file, File};
use std::io::{Read, Write};
use clap::{command, Parser};
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::io::bitstream::{c_bitstream_writer, e_bitstream_byte_order};
use blf_lib::blf::chunks::find_chunk_in_file;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_map_variant, s_blf_chunk_packed_map_variant};
use crate::commands::Commands;
use crate::commands::import_rsa_signature::import_rsa_signature;
use crate::title_storage::halo3::release::blf_files::map_variant::map_variant;
use blf_lib::blf::BlfFile;
use crate::commands::import_map_variant::import_map_variant;

mod title_storage;
mod io;
mod console;
mod commands;

#[derive(Debug, Parser)]
#[command(name = "blf_cli")]
#[command(about = "blam! engine file editor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::BuildTitleStorage { config_input_path, blf_output_path, title, version } => {
            let mut title_converter =
                title_storage::get_title_converter(title, version)
                    .expect("No title converter was found for the provided title and version.");

            title_converter.build_blfs(
                &config_input_path,
                &blf_output_path
            );
        },
        Commands::BuildTitleStorageConfig { blf_input_path, config_output_path, title, version } => {
            let mut title_converter =
                title_storage::get_title_converter(title, version)
                    .expect("No title converter was found for the provided title and version.");

            title_converter.build_config(
                &blf_input_path,
                &config_output_path
            );
        },
        Commands::ImportRsaSignature { config_path, map_file_path, title, version } => {
            import_rsa_signature(config_path, map_file_path, title, version);
        }
        Commands::ImportMapVariant { config_path, map_variant_file_path, title, version} => {
            import_map_variant(config_path, map_variant_file_path, title, version);
        }
    }
}