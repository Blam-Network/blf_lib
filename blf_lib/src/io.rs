use std::error::Error;
use std::fs::File;
use std::io::Read;
use serde::de::DeserializeOwned;
pub use blf_lib_derivable::io::*;

pub mod packed_decoding;
pub mod packed_encoding;
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