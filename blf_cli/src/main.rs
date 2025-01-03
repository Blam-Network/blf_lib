#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{command, Parser};
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

fn main() {
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