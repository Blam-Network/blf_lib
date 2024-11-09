use std::fs::File;
use std::io::Write;
use blf_lib::blam::common::cache::cache_files::s_cache_file_header_v11;
use blf_lib::blf::chunks::find_chunk_in_file;
use blf_lib::blf::versions::halo3::k_title_halo3;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_scenario;
use blf_lib::blf::versions::halo3odst::k_title_halo3odst;
use blf_lib::TEST_BIT;
use crate::build_path;
use crate::console::console_task;
use crate::io::{create_parent_folders, get_files_in_folder};

pub fn import_rsa_signatures(
    hoppers_config_path: String,
    halo_maps_folder: String,
    title: String,
    version: String,
) {
    let mut task = console_task::start(String::from("Importing RSA Signatures"));

    if title == k_title_halo3 || title == k_title_halo3odst {

        let map_info_folder = build_path!(
            &halo_maps_folder,
            "info"
        );

        let map_info_file_names = get_files_in_folder(&map_info_folder).unwrap_or_else(|err|{
            task.fail_with_error(err);
            panic!()
        });

        let rsa_signatures_config_path = build_path!(
            &hoppers_config_path,
            "rsa_signatures"
        );

        let mut multiplayer_maps = Vec::<(u32, String)>::new();

        for map_info_file_name in map_info_file_names {
            if !map_info_file_name.ends_with(".mapinfo") {
                continue;
            }

            let map_info_file_path = build_path!(
                &map_info_folder,
                &map_info_file_name
            );

            let scenario_chunk = find_chunk_in_file::<s_blf_chunk_scenario>(&map_info_file_path);

            if scenario_chunk.is_err() {
                task.add_error(format!("Failed to read {map_info_file_name}"));
            }

            let scenario_chunk = scenario_chunk.unwrap();

            // Multiplayer or Mainmenu
            if TEST_BIT!(scenario_chunk.map_flags, 6) || TEST_BIT!(scenario_chunk.map_flags, 4) {
                multiplayer_maps.push((scenario_chunk.map_id ,scenario_chunk.scenario_path.get_string()))
            }
        }

        for (map_id, map_file_name) in multiplayer_maps {
            let map_file_path = build_path!(
                &halo_maps_folder,
                format!("{map_file_name}.map")
            );


            let cache_file = s_cache_file_header_v11::read(map_file_path);
            if cache_file.is_err() {
                task.add_error(format!("Failed to read {map_file_name}.map"));
                return;
            }

            let cache_file = cache_file.unwrap();

            let output_file_path = build_path!(
                &rsa_signatures_config_path,
                format!("{map_id}_{map_file_name}")
            );

            task.add_message(format!("{map_id}_{map_file_name}"));

            create_parent_folders(&output_file_path).unwrap();
            let mut output_file = File::create(output_file_path).unwrap();
            output_file.write_all(cache_file.rsa_signature.get()).unwrap();
        }
    } else {
        task.add_error("Unsupported title and version.");

        task.fail();
        return;
    }

    task.complete();
}