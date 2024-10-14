#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{command, Parser, Subcommand};

mod title_storage;
mod io;
mod console;

#[derive(Debug, Parser)]
#[command(name = "blf_cli")]
#[command(about = "blam! engine file editor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    BuildTitleStorage {
        input: String,
        output: String,
        title: String,
        version: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::BuildTitleStorage { input, output, title, version } => {
            let mut title_converter =
                title_storage::get_title_converter(title, version)
                    .expect("No title converter was found for the provided title and version.");

            title_converter.build_blfs(
                &input,
                &output
            );
        }
    }
}