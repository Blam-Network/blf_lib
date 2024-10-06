use std::ffi::c_char;
use blf_lib_derive::BlfChunk;
use blf_lib_derivable::blf::chunks::BlfChunk;

const k_tag_string_length: usize = 32;

#[repr(C, align(4))]
#[derive(BlfChunk)]
#[Signature("_blf")]
#[Version(1.2)]
struct s_blf_chunk_start_of_file
{
    byte_order_mark: u16,

    name: [c_char; k_tag_string_length],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizeof_s_blf_chunk_start_of_file() {
        assert_eq!(size_of::<s_blf_chunk_start_of_file>(), 0x24);
    }

    // These are more tests of the blf_lib-derive package.
    // They don't really belong here, we'll move them in future.
    #[test]
    fn s_blf_chunk_start_of_file_signature() {
        assert_eq!(s_blf_chunk_start_of_file::get_signature(), ['_' as c_char, 'b' as c_char, 'l' as c_char, 'f' as c_char]);
    }

    #[test]
    fn s_blf_chunk_start_of_file_version() {
        assert_eq!(s_blf_chunk_start_of_file::get_version(), [1, 2]);
    }
}

