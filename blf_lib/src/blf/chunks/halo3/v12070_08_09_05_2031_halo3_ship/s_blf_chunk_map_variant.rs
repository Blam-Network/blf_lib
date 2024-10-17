use blf_lib::blam::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf_chunk;

blf_chunk!(
    #[Signature("mapv")]
    #[Version(12.1)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_map_variant
    {
        #[serde(skip_serializing)] pad1: u32,
        map_variant: c_map_variant,
        #[serde(skip_serializing)] pad2: u32,
    }
);

impl s_blf_chunk_map_variant {
}