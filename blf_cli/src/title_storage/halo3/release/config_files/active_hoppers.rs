use std::fs::File;
use std::io::Read;
use crate::build_path;

const active_hoppers_file_name: &str = "active_hoppers.txt";

pub fn read_active_hoppers(hoppers_config_folder: &String) -> Result<Vec<String>, String> {
    let active_hoppers_file_path = build_path!(
        hoppers_config_folder,
        active_hoppers_file_name
    );

    let active_hoppers_file = File::open(&active_hoppers_file_path);
    if active_hoppers_file.is_err() {
        return Err(active_hoppers_file.unwrap_err().to_string());
    }

    let mut active_hoppers_file = active_hoppers_file.unwrap();
    let mut active_hoppers_string = String::new();
    let read_result = active_hoppers_file.read_to_string(&mut active_hoppers_string);
    if read_result.is_err() {
        return Err(read_result.unwrap_err().to_string());
    }

    let active_hopper_folders = active_hoppers_string.lines();

    Ok(active_hopper_folders.map(|thing|String::from(thing)).collect::<Vec<String>>())
}