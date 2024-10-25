use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_territories_variant {
    m_variant_flags: u16,
    m_respawn_on_capture: u16,
    m_capture_time: u16,
    m_sudden_death_time: u16,
    m_defender_traits: c_player_traits,
    m_attacker_traits: c_player_traits,
}