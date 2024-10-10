use bincode::{Decode, Encode};

#[derive(Clone, Copy, Default, Debug, Encode, Decode, PartialEq)]
pub struct chunk_version {
    pub major: u16,
    pub minor: u16,
}

impl chunk_version {
    pub fn new(version_float: f32) -> chunk_version {
        if version_float >= 100f32 {
            panic!("Invalid chunk version!")
        }
        let version_int = (version_float * 10.0) as u32;
        let major = (version_int / 10) as u16;
        let minor = (version_int % 10) as u16;

        chunk_version {
            major,
            minor,
        }
    }
}

