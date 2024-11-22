pub mod import_rsa_signatures;
pub mod import_variant;
pub mod export_variant;
pub mod title_storage;
pub mod convert_halo3mcc_map_variants;

use clap::Subcommand;
use crate::commands::title_storage::TitleStorageCommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "title-storage")]
    TitleStorage(TitleStorageCommand),
    #[command(arg_required_else_help = true)]
    ConvertH3MCCMapVariants {
        mcc_maps_folder: String,
        converted_maps_folder: String,
    },
}