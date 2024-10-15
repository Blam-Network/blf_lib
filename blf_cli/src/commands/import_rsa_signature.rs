use std::fs::{create_dir_all, File};
use std::io::Write;
use blf_lib::blam::cache::cache_files::s_cache_file_header_v11;
use blf_lib::blf::versions::halo3::k_title_halo3;
use crate::console::console_task;
use crate::io::{build_path, get_directories_in_folder};
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::k_build_string_halo3_ship_12070;

pub fn import_rsa_signature(
    config_path: String,
    map_file_path: String,
    title: String,
    version: String,
) {
    let mut task = console_task::start(String::from("Importing RSA Signature"));

    if version == k_build_string_halo3_ship_12070 && title == k_title_halo3 {
        let cache_file = s_cache_file_header_v11::read(map_file_path);
        if cache_file.is_err() {
            task.add_error(cache_file.unwrap_err());
            task.complete();
            return;
        }

        let mut cache_file = cache_file.unwrap();

        let hopper_directories = get_directories_in_folder(&config_path);

        for hopper_directory in hopper_directories {
            let output_folder_path = build_path(vec![
                &config_path,
                &hopper_directory,
                &"rsa_signatures".to_string(),
            ]);

            let output_file_path = build_path(vec![
                &output_folder_path,
                &cache_file.map_name.get_string(),
            ]);

            create_dir_all(&output_folder_path).unwrap();

            let mut output_file = File::create(output_file_path).unwrap();
            output_file.write_all(cache_file.rsa_signature.get()).unwrap();
        }



    } else {
        task.add_error("Unsupported title and version.".to_string());
    }

    task.complete();
}