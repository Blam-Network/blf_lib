use std::ffi::c_char;

pub trait BlfChunk {

    fn get_signature() -> [c_char; 4];
    fn get_version() -> [u16; 2];
}
