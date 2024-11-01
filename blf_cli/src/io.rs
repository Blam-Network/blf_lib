use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::MAIN_SEPARATOR_STR;
use serde::{Serialize, Serializer};
use crate::title_storage::check_file_exists;

pub fn get_directories_in_folder(path: &String) -> Result<Vec<String>, String> {
    if !check_file_exists(path) {
        return Err(format!("Folder not found: {}", path))
    }
    Ok(fs::read_dir(path).unwrap()
        .filter(|res|res.as_ref().unwrap().metadata().unwrap().is_dir())
        .map(|res| res.map(|e| e.file_name().to_str().unwrap().to_string()))
        .collect::<Result<Vec<_>, std::io::Error>>().unwrap())
}

pub fn get_files_in_folder(path: &String) -> Result<Vec<String>, String> {
    if !check_file_exists(path) {
        return Err(format!("Folder not found: {}", path))
    }
    Ok(fs::read_dir(path).unwrap()
        .filter(|res|res.as_ref().unwrap().metadata().unwrap().is_file())
        .map(|res| res.map(|e| e.file_name().to_str().unwrap().to_string()))
        .collect::<Result<Vec<_>, std::io::Error>>().unwrap())
}

pub const FILE_SEPARATOR: &str = MAIN_SEPARATOR_STR;

pub fn build_path(parts: Vec<&String>) -> String {
    parts.iter().map(|part| part.to_string()).collect::<Vec<String>>().join(FILE_SEPARATOR)
}

/// For use with serde's [serialize_with] attribute
pub fn ordered_map<S, K: Ord + Serialize, V: Serialize>(
    value: &HashMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}