use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use crate::types::array::Array;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_infection_variant {
    m_variant_flags: u8,
    m_safe_havens: u8,
    m_next_zombie: u8,
    m_initial_zombie_count: u8,
    m_safe_haven_movement_time: u16,
    m_zombie_kill_points: u8,
    m_infection_points: u8,
    m_safe_haven_arrival_points: u8,
    m_suicide_points: u8,
    m_betrayal_points: u8,
    m_last_man_bonus_points: u8,
    m_zombie_traits: c_player_traits,
    m_first_zombie_traits: c_player_traits,
    m_safe_haven_defender_traits: c_player_traits,
    m_last_human_traits: c_player_traits,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad1: Array<u8, 4>,
}