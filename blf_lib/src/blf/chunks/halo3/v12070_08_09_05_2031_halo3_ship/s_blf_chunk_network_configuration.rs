use std::ffi::c_char;
use crate::blf_chunk;
use crate::types::byte_order_mark::byte_order_mark;
use crate::types::c_string::from_string_with_length;

blf_chunk!(
    #[Signature("netc")]
    #[Version(135.1)]
    #[Size(8300)]
    #[PackedSerialize(1, LittleEndian)]
    pub struct s_blf_chunk_network_configuration
    {
        // TODO: Map
        data: [u8; 8300],
    }
);