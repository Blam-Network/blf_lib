use blf_lib::types::array::StaticArray;
use crate::blf_chunk;

blf_chunk!(
    #[Signature("netc")]
    #[Version(128.1)]
    #[PackedSerialize(1, LittleEndian)]
    pub struct s_blf_chunk_network_configuration
    {
        // TODO: Map
        data: StaticArray<u8, 5568>,
    }
);
