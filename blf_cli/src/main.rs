#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{command, Parser, Subcommand};
use crate::commands::Commands;
use crate::commands::import_rsa_signature::import_rsa_signature;

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
        Commands::ImportRsaSignature { config_path, map_file_path, title, version } => {
            import_rsa_signature(config_path, map_file_path, title, version);
        }
    }
}