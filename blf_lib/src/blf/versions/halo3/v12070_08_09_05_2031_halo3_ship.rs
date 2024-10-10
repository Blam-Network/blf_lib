use blf_lib_derive::ChunkFactory;
use crate::blf::chunks::halo3;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::s_blf_chunk_start_of_file;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_author::s_blf_chunk_author;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::s_blf_chunk_end_of_file;

#[derive(ChunkFactory)]
#[Title("Halo 3")]
#[Build("12070.08.09.05.2031.halo3_ship")]
#[Chunks(
    s_blf_chunk_start_of_file, 
    s_blf_chunk_author
)]
pub struct v12070_08_09_05_2031_halo3_ship {}

// TODO: Move test
#[cfg(test)]
mod tests {
    use crate::types::c_string::to_string;
    use blf_lib_derivable::{blf::chunks::ChunkFactory, types::chunk_signature::chunk_signature};
    use blf_lib_derivable::types::chunk_version::chunk_version;
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

        let result = version.decode(&chunk_signature::from_string("_blf"), chunk_version::new(1.2), &data);
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

