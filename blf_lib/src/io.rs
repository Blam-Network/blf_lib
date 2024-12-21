use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod bitstream;

// Consider returning io error instead of generic.
pub fn read_file_to_string(path: impl Into<String>) -> Result<String, Box<dyn Error>> {
    let path = path.into();
    let mut file = File::open(&path).map_err(|err|{
        Box::<dyn Error>::from(format!("read_file_to_string({path}) {}", err))
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_json_file<T: DeserializeOwned>(path: impl Into<String>) -> Result<T, Box<dyn Error>> {
    let json = read_file_to_string(path)?;
    serde_json::from_str(&json).map_err(|e| e.into())
}

pub fn write_json_file<T: Serialize>(value: &T, path: impl Into<String>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(value).unwrap();
    let mut text_file = File::create(path.into()).unwrap();
    text_file.write_all(json.as_bytes())?;
    Ok(())
}