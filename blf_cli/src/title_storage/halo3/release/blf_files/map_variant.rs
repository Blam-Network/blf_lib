use std::error::Error;
use std::io::Write;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_packed_map_variant, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;
use crate::build_path;
use crate::io::create_parent_folders;

pub const k_map_variants_config_folder_name: &str = "map_variants";
pub const k_map_variants_blf_folder_name: &str = "map_variants";


blf_file! {
    pub struct map_variant {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mpvr: s_blf_chunk_packed_map_variant,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl map_variant {
    pub fn create(map_variant: c_map_variant) -> map_variant {
        map_variant {
            _blf: s_blf_chunk_start_of_file::new("map variant", byte_order_mark::default()),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            mpvr: s_blf_chunk_packed_map_variant::create(map_variant),
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }

    pub fn write_to_config(&self, hoppers_config_path: &String, file_name: &str) -> Result<(), Box<dyn Error>> {
        let config_file_path = build_path!(
            hoppers_config_path,
            k_map_variants_config_folder_name,
            format!("{file_name}.json")
        );

        create_parent_folders(&config_file_path)?;
        let map_variant_json = serde_json::to_string_pretty(&self.mpvr.map_variant)?;
        let mut map_variant_json_file = File::create_new(config_file_path)?;
        map_variant_json_file.write_all(map_variant_json.as_bytes())?;

        Ok(())
    }
}