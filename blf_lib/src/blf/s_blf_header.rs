use std::ffi::c_char;

#[repr(C, packed)]
struct s_blf_header
{
    chunk_type: [c_char; 4],
    chunk_size: u32,
    major_version: u16,
    minor_version: u16,
}

impl s_blf_header {
    pub fn setup(&mut self, chunk_type: [c_char; 4], chunk_size: u32, major_version: u16, minor_version: u16) {
        self.chunk_type = chunk_type;
        self.chunk_size = chunk_size;
        self.major_version = major_version;
        self.minor_version = minor_version;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sizeof_s_blf_header() {
        assert_eq!(size_of::<s_blf_header>(), 0xC);
    }
}