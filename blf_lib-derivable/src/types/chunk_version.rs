use std::fmt::Display;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct chunk_version {
    pub major: u16,
    pub minor: u16,
}

impl chunk_version {
    pub fn new(version_float: f32) -> chunk_version {
        let version_int = (version_float * 10.0) as u32;
        let major = (version_int / 10) as u16;
        let minor = (version_int % 10) as u16;

        chunk_version {
            major,
            minor,
        }
    }
}

impl Display for chunk_version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}
