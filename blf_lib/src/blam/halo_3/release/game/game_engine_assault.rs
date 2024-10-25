use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;
use crate::types::array::Array;
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_assault_variant {
    m_variant_flags: u16,
    m_respawn: u16,
    m_game_type: u16,
    m_enemy_bomb_waypoint: u16,
    m_score_to_win: u16,
    m_sudden_death_time: u16,
    m_bomb_reset_time: u16,
    m_bomb_arming_time: u16,
    m_bomb_disarming_time: u16,
    m_bomb_fuse_time: u16,
    m_carrier_traits: c_player_traits,
    m_arming_traits: c_player_traits,
    #[serde(skip_serializing,skip_deserializing)]
    m_pad1: Array<u8, 4>,
}