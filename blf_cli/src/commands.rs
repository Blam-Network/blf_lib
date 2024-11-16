pub mod import_rsa_signatures;
pub mod import_variant;
pub mod export_variant;
pub mod title_storage;

use clap::Subcommand;
use crate::commands::title_storage::TitleStorageCommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "title-storage")]
    TitleStorage(TitleStorageCommand),
}