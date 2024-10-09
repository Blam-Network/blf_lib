use std::ffi::c_char;
use bincode::Encode;
use blf_lib_derive::TestSize;
use blf_lib_derive::{BlfChunk, UnpackedSerializable};
use crate::types::byte_order_mark::byte_order_mark;
use crate::types::c_string::from_string;

const k_tag_string_length: usize = 32;

#[derive(BlfChunk, Default, UnpackedSerializable, TestSize, PartialEq, Debug)]
#[Signature("_blf")]
#[Version(1.2)]
#[Size(0x24)]
#[LittleEndian]
#[Pack(1)]
pub struct s_blf_chunk_start_of_file
{
    pub byte_order_mark: byte_order_mark,
    pub name: [c_char; k_tag_string_length],
    pub __data: [c_char; 2],
}

impl s_blf_chunk_start_of_file {
    pub fn new(name: &str, byte_order_mark: byte_order_mark) -> s_blf_chunk_start_of_file {
        s_blf_chunk_start_of_file {
            byte_order_mark,
            name: from_string(name.to_string(), 32).try_into().unwrap(),
            __data: [0; 2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::byte_order_mark;
    use crate::types::c_string::{from_string, to_string};
    use blf_lib_derivable::blf::chunks::BlfChunk;
    use blf_lib_derivable::blf::chunks::Serializable;
    use blf_lib_derivable::types::chunk_signature::chunk_signature;
    use blf_lib_derivable::types::chunk_version::chunk_version;

    // These are more tests of the blf_lib-derive package.
    // They don't really belong here, we'll move them in future.
    #[test]
    fn s_blf_chunk_start_of_file_signature() {
        assert_eq!(s_blf_chunk_start_of_file::get_signature(), chunk_signature::from_string("_blf"));
    }

    #[test]
    fn s_blf_chunk_start_of_file_version() {
        assert_eq!(s_blf_chunk_start_of_file::get_version(), chunk_version::new(1.2));
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
        let bom = start_of_file.byte_order_mark;
        assert_eq!(bom, byte_order_mark::little_endian);
        assert_eq!(to_string(&start_of_file.name), "halo3 multiplayer");
    }
}

