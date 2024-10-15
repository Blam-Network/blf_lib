use std::ffi::c_char;
use blf_lib::types::array::Array;
use crate::blf_chunk;

blf_chunk!(
    #[Signature("netc")]
    #[Version(135.1)]
    #[Size(8300)]
    #[PackedSerialize(1, LittleEndian)]
    pub struct s_blf_chunk_network_configuration
    {
        // TODO: Map
        data: Array<u8, 8300>,
    }
);