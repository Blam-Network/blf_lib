pub mod import_rsa_signature;

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
    ImportRsaSignature {
        config_path: String,
        map_file_path: String,
        title: String,
        version: String,
    },
}