use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about = "Manage title storage.")]
pub struct TitleStorageCommand {
    #[command(subcommand)]
    pub command: TitleStorageSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum TitleStorageSubcommands {
    #[command(arg_required_else_help = true)]
    Build {
        config_input_path: String,
        blf_output_path: String,
        title: String,
        version: String,
    },
    #[command(arg_required_else_help = true)]
    BuildConfig {
        blf_input_path: String,
        config_output_path: String,
        title: String,
        version: String,
    },
    #[command(arg_required_else_help = true)]
    ImportRsaSignatures {
        hoppers_config_path: String,
        halo_maps_folder: String,
        title: String,
        version: String,
    },
    #[command(arg_required_else_help = true)]
    ImportVariant {
        hoppers_config_path: String,
        variant_path: String,
        title: String,
        version: String,
    },
    #[command(arg_required_else_help = true)]
    ExportVariant {
        variant_json_path: String,
        destination_path: String,
        title: String,
        version: String,
    },
}