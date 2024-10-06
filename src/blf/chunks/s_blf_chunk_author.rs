use std::ffi::c_char;
use crate::blf::chunks::{BlfChunk, ReturnsSignature};

struct s_blf_chunk_author
{
    build_name: [c_char; 16],
    build_identifier: u64,
    build_string: [c_char; 28],
    author_name: [c_char; 16],
}

impl BlfChunk for s_blf_chunk_author {

    fn get_signature() -> [c_char; 4] {
        "athr".to_signature()
    }

    fn get_version() -> [u16; 2] {
        [3, 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sizeof_s_blf_chunk_author() {
        assert_eq!(size_of::<s_blf_chunk_author>(), 0x44);
    }
}
