use std::ffi::c_char;
use blf_lib::blf_chunk;
use crate::types::build_number_identifier::build_number_identifier;
use crate::types::c_string::from_string_with_length;

blf_chunk!(
    #[Signature("athr")]
    #[Version(3.1)]
    #[Size(0x44)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_author
    {
        pub build_name: [c_char; 16],
        pub build_identifier: build_number_identifier,
        pub build_string: [c_char; 28],
        pub author_name: [c_char; 16],
    }
);

impl s_blf_chunk_author {
    pub fn new(build_name: &str, build_identifier: build_number_identifier, build_string: &str, author_name: &str) -> s_blf_chunk_author {
        s_blf_chunk_author {
            build_name: from_string_with_length(build_name.to_string(), 16).try_into().unwrap(),
            build_identifier,
            build_string: from_string_with_length(build_string.to_string(), 28).try_into().unwrap(),
            author_name: from_string_with_length(author_name.to_string(), 16).try_into().unwrap(),
        }
    }
}
