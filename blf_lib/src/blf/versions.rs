use blf_lib_derive::VersionFactory;

pub use halo3::v12070_08_09_05_2031_halo3_ship::v12070_08_09_05_2031_halo3_ship;

pub mod halo3;

// Macro manages the implementation.
#[derive(VersionFactory)]
#[Versions(
    v12070_08_09_05_2031_halo3_ship
)]
pub struct version_factory {}