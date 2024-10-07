use std::ffi::c_char;
use blf_lib_derive::ChunkFactory;
use blf_lib_derivable::blf::chunks::{BlfChunk, Serializable};
use crate::blf::chunks::halo3;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::s_blf_chunk_start_of_file;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_author::s_blf_chunk_author;

#[derive(ChunkFactory)]
#[Title("Halo 3")]
#[Build("12070.08.09.05.2031.halo3_ship")]
#[Chunks(
    s_blf_chunk_start_of_file, 
    s_blf_chunk_author
)]
struct v12070_08_09_05_2031_halo3_ship {}

#[cfg(test)]
mod tests {
    use crate::types::c_string::to_string;
    use blf_lib_derivable::blf::chunks::ChunkFactory;

    use super::*;

    #[test]
    fn test_the_factory() {
        let version = v12070_08_09_05_2031_halo3_ship {};

        let data: [u8; 36] = [
                                    0xFF, 0xFE, 0x68, 0x61,
            0x6C, 0x6F, 0x33, 0x20, 0x6D, 0x75, 0x6C, 0x74,
            0x69, 0x70, 0x6C, 0x61, 0x79, 0x65, 0x72, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];

        let result = version.decode_chunk(&['_' as c_char, 'b' as c_char, 'l' as c_char, 'f' as c_char], 1, 2, &data);
        let boxed = result.unwrap();
        
        // Editing dynamic chunks isn't yet supported.
        // So for the sake of testing we perform an unsafe cast to check the contents.
        
        let chunk_ptr = Box::into_raw(boxed) as *mut s_blf_chunk_start_of_file;
        let chunk = unsafe { &*chunk_ptr };

        assert_eq!(to_string(&chunk.name), "halo3 multiplayer");

        unsafe {
            drop(Box::from_raw(chunk_ptr));
        }
    }
}

