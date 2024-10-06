use std::ffi::c_char;

#[repr(C, packed)]
#[derive(Default)]
pub struct s_blf_header
{
    pub chunk_type: [c_char; 4],
    pub chunk_size: u32,
    pub major_version: u16,
    pub minor_version: u16,
}

impl s_blf_header {
    pub fn setup(chunk_type: [c_char; 4], chunk_size: u32, version: [u16; 2]) -> s_blf_header {
        s_blf_header {
            chunk_type,
            chunk_size,
            major_version: version[0],
            minor_version: version[1],
        }
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