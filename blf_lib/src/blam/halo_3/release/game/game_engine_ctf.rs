use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_ctf_variant {
    m_variant_flags: u8,
    m_home_flag_waypoint: u8,
    m_game_type: u8,
    m_respawn: u8,
    m_touch_return_timeout: u16,
    m_sudden_death_time: u16,
    m_score_to_win: u16,
    m_flag_reset_time: u16,
    m_carrier_traits: c_player_traits,
}