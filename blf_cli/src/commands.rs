pub mod import_rsa_signatures;
pub mod import_variant;
pub mod export_variant;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    BuildTitleStorage {
        config_input_path: String,
        blf_output_path: String,
        title: String,
        version: String,
    },
    #[command(arg_required_else_help = true)]
    BuildTitleStorageConfig {
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