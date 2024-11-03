use blf_lib::blf::versions::halo3::k_title_halo3;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::k_build_string_halo3_ship_12070;

pub fn import_variant(
    hoppers_config_path: String,
    variant_path: String,
    title: String,
    version: String,
) {
    if version == k_build_string_halo3_ship_12070 && title == k_title_halo3 {
        v12070_08_09_05_2031_halo3_ship::variant_importer::import_variant(&hoppers_config_path, &variant_path);
    } else {
        println!("Unsupported title or version.");
    }
}