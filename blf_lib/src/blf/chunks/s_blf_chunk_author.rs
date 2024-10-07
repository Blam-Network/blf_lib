use std::ffi::c_char;
use blf_lib_derive::{BlfChunk, TestSize};

#[repr(C, packed)]
#[derive(BlfChunk, TestSize)]
#[Signature("athr")]
#[Version(3.1)]
#[Size(0x44)]
pub struct s_blf_chunk_author
{
    build_name: [c_char; 16],
    build_identifier: u64,
    build_string: [c_char; 28],
    author_name: [c_char; 16],
}

#[cfg(test)]
mod tests {
    use blf_lib_derivable::blf::chunks::BlfChunk;
    use super::*;

    // These are more tests of the blf_lib-derive package.
    // They don't really belong here, we'll move them in future.
    #[test]
    fn s_blf_chunk_author_signature() {
        assert_eq!(s_blf_chunk_author::get_signature(), ['a' as c_char, 't' as c_char, 'h' as c_char, 'r' as c_char]);
    }

    #[test]
    fn s_blf_chunk_author_version() {
        assert_eq!(s_blf_chunk_author::get_version(), [3, 1]);
    }
}
