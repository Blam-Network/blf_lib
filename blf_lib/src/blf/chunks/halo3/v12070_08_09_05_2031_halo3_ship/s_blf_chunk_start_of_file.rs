use std::ffi::c_char;
use blf_lib_derive::TestSize;
use blf_lib_derive::{BlfChunk, UnpackedSerializable};
use blf_lib_derivable::blf::chunks::BlfChunk;
use blf_lib_derivable::blf::chunks::Serializable;

const k_tag_string_length: usize = 32;

#[repr(C, align(4))]
#[derive(BlfChunk, Default, UnpackedSerializable, TestSize)]
#[Signature("_blf")]
#[Version(1.2)]
#[Size(0x24)]
pub struct s_blf_chunk_start_of_file
{
    pub byte_order_mark: u16,

    pub name: [c_char; k_tag_string_length],
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::c_string::to_string;

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

    #[test]
    fn decode_start_of_file() {
        let data: [u8; 36] = [
                                    0xFF, 0xFE, 0x68, 0x61,
            0x6C, 0x6F, 0x33, 0x20, 0x6D, 0x75, 0x6C, 0x74,
            0x69, 0x70, 0x6C, 0x61, 0x79, 0x65, 0x72, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];
        let start_of_file = s_blf_chunk_start_of_file::decode(&data);
        assert_eq!(start_of_file.byte_order_mark, 0xFEFF);
        assert_eq!(to_string(&start_of_file.name), "halo3 multiplayer");
    }
}
