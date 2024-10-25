use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_sandbox_variant {
    m_variant_flags: u8,
    m_edit_mode: u8,
    m_respawn_time: u16,
    m_player_traits: c_player_traits,
}