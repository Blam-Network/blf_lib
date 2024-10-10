use std::ffi::c_char;
use crate::blf_chunk;
use crate::types::byte_order_mark::byte_order_mark;
use crate::types::c_string::from_string;

const k_tag_string_length: usize = 32;

blf_chunk!(
    #[Signature("_blf")]
    #[Version(1.2)]
    #[Size(0x24)]
    #[PackedEncode(1, LittleEndian)]
    pub struct s_blf_chunk_start_of_file
    {
        pub byte_order_mark: byte_order_mark,
        pub name: [c_char; k_tag_string_length],
        pub __data: [c_char; 2],
    }
);

impl s_blf_chunk_start_of_file {
    pub fn new(name: &str, byte_order_mark: byte_order_mark) -> s_blf_chunk_start_of_file {
        s_blf_chunk_start_of_file {
            byte_order_mark,
            name: from_string(name.to_string(), 32).try_into().unwrap(),
            __data: [0; 2],
        }
    }
}