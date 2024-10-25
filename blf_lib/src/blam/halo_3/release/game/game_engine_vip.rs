use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use crate::types::array::Array;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_vip_variant {
    m_score_to_win_round: u16,
    m_variant_flags: u16,
    m_kill_points: u8,
    m_takedown_points: u8,
    m_kill_as_vip_points: u8,
    m_vip_death_points: u8,
    m_destination_arrival_points: u8,
    m_suicide_points: u8,
    m_betrayal_points: u8,
    m_vip_suicide_points: u8,
    m_vip_selection: u8,
    m_zone_movement: u8,
    m_zone_order: u8,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad1: Array<u8, 1>,
    m_influence_radius: u16,
    m_vip_team_traits: c_player_traits,
    m_vip_influence_traits: c_player_traits,
    m_vip_traits: c_player_traits,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad2: Array<u8, 2>,
}