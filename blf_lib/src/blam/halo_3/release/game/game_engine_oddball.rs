use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use crate::types::array::Array;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_oddball_variant {
    m_variant_flags: u32,
    m_score_to_win: u16,
    m_carrying_points: u16,
    m_kill_points: u8,
    m_ball_kill_points: u8,
    m_carrier_kill_points: u8,
    m_ball_count: u8,
    m_ball_spawn_delay: u16,
    m_ball_inactive_respawn_delay: u16,
    m_carrier_traits: c_player_traits,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad1: Array<u8, 2>,
}