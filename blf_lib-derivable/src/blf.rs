pub mod chunks;
pub mod s_blf_header;

pub trait BlfFile {
    fn write(&mut self, path: &String);
    fn read(path: &String) -> Self;
}