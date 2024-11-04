use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blf_chunk;

blf_chunk!(
    #[Signature("mpvr")]
    #[Version(3.1)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_game_variant
    {
        pub game_variant: c_game_variant,
    }
);

impl s_blf_chunk_game_variant {
    pub fn create(game_variant: c_game_variant) -> s_blf_chunk_game_variant {
        s_blf_chunk_game_variant { game_variant }
    }
}