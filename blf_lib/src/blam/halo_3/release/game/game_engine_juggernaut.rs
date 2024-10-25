use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use crate::types::array::Array;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_juggernaut_variant {
    m_score_to_win_round: u16,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad1: Array<u8, 2>,
    m_initial_juggernaut: u8,
    m_next_juggernaut: u8,
    m_variant_flags: u8,
    m_zone_movement: u8,
    m_zone_order: u8,
    m_kill_points: u8,
    m_juggernaut_kill_points: u8,
    m_kill_as_juggernaut_points: u8,
    m_destination_arrival_points: u8,
    m_suicide_points: u8,
    m_betrayal_points: u8,
    m_juggernaut_delay: u8,
    m_juggernaut_traits: c_player_traits,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad2: Array<u8, 4>,
}