use blf_lib_derive::VersionFactory;

pub use halo3::v12070_08_09_05_2031_halo3_ship::v12070_08_09_05_2031_halo3_ship;
pub use halo3odst::v13895_09_04_27_2201_atlas_release::v13895_09_04_27_2201_atlas_release;

pub mod halo3;
pub mod halo3odst;

// Macro manages the implementation.
#[derive(VersionFactory)]
#[Versions(
    v12070_08_09_05_2031_halo3_ship,
    v13895_09_04_27_2201_atlas_release,
)]
pub struct version_factory {}