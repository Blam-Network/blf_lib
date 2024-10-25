use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use crate::types::array::Array;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_king_variant {
    m_variant_flags: u32,
    m_score_to_win: u16,
    m_moving_hill: u8,
    m_moving_hill_order: u8,
    m_uncontested_hill_bonus: u8,
    m_kill_points: u8,
    m_inside_hill_points: u8,
    m_outside_hill_points: u8,
    m_inside_hill_traits: c_player_traits,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad1: Array<u8, 6>,
}