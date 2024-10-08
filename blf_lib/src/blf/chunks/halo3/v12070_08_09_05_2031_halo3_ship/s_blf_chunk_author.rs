use std::ffi::c_char;
use bincode::Encode;
use blf_lib_derive::{BlfChunk, TestSize, UnpackedSerializable};
use crate::types::build_number_identifier::build_number_identifier;

#[repr(C, packed(4))]
#[derive(BlfChunk, UnpackedSerializable, TestSize, Debug)]
#[Signature("athr")]
#[Version(3.1)]
#[Size(0x44)]
#[BigEndian]
#[Pack(1)]
pub struct s_blf_chunk_author
{
    pub build_name: [c_char; 16],
    pub build_identifier: build_number_identifier,
    pub build_string: [c_char; 28],
    pub author_name: [c_char; 16],
}

#[cfg(test)]
mod tests {
    use blf_lib_derivable::{blf::chunks::BlfChunk, types::chunk_signature::chunk_signature};
    use crate::types::c_string::{from_string, to_string};
    use super::*;
    use blf_lib_derivable::blf::chunks::Serializable;
    use crate::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file;

    // These are more tests of the blf_lib-derive package.
    // They don't really belong here, we'll move them in future.
    #[test]
    fn s_blf_chunk_author_signature() {
        assert_eq!(s_blf_chunk_author::get_signature(), chunk_signature::from_string("athr"));
    }

    #[test]
    fn s_blf_chunk_author_version() {
        assert_eq!(s_blf_chunk_author::get_version(), [3, 1]);
    }

    #[test]
    fn encode_and_decode() {
        let expected: [u8; 0x44] = [
                                                                                    0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x2F, 0x26, 0x31, 0x32, 0x30, 0x37, 0x30, 0x2E, 0x30, 0x38, 0x2E, 0x30, 0x39, 0x2E,
            0x30, 0x35, 0x2E, 0x32, 0x30, 0x33, 0x31, 0x2E, 0x68, 0x61, 0x6C, 0x6F, 0x33, 0x5F, 0x73, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];


        let author = s_blf_chunk_author {
            build_name: [0; 16],
            build_identifier: build_number_identifier::new(1, 12070),
            build_string: from_string("12070.08.09.05.2031.halo3_s".to_string(), 28).try_into().unwrap(),
            author_name: [0; 16],
        };

        let encoded = author.encode_chunk();
        let decoded  = s_blf_chunk_author::decode_chunk(&encoded);

        assert_eq!(encoded, expected);
        assert_eq!(to_string(&decoded.build_string), to_string(&author.build_string));
        let read_build_ident = decoded.build_identifier;
        let expected_build_ident = author.build_identifier;
        assert_eq!(read_build_ident, expected_build_ident);
    }

    #[derive(BlfChunk, UnpackedSerializable, Debug, TestSize)]
    #[Signature("test")]
    #[Version(0.0)]
    #[BigEndian]
    #[Pack(4)]
    #[Size(8)]
    struct padding_test {
        pub val1: u8,
        pub val2: u8,
    }

    #[test]
    fn encode_and_decode_test_struct() {
        let expected: [u8; 0x8] = [
            0x11, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, 0x00,
        ];

        let input = padding_test {
            val1: 0x11,
            val2: 0x22,
        };

        let encoded = input.encode_chunk();
        let decoded = padding_test::decode_chunk(&encoded);

        assert_eq!(encoded, expected);
        assert_eq!(decoded.val1, input.val1);
        assert_eq!(decoded.val2, input.val2);

    }
}
