use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_slayer_variant {
    m_score_to_win: u16,
    m_kill_points: u16,
    m_assist_points: u8,
    m_death_points: u8,
    m_suicide_points: u8,
    m_betrayal_points: u8,
    m_leader_killed_points: u8,
    m_elimination_points: u8,
    m_assassination_points: u8,
    m_headshot_points: u8,
    m_melee_points: u8,
    m_sticky_points: u8,
    m_splatter_points: u8,
    m_killing_spree_points: u8,
}