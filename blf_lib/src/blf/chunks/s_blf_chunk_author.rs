use std::ffi::c_char;
use blf_lib_derive::BlfChunk;

#[repr(C, packed)]
#[derive(BlfChunk)]
#[Signature("athr")]
#[Version(3.1)]
struct s_blf_chunk_author
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
    #[test]
    fn sizeof_s_blf_chunk_author() {
        assert_eq!(size_of::<s_blf_chunk_author>(), 0x44);
    }

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
