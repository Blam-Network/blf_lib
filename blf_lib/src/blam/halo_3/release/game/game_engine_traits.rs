use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib_derive::PackedSerialize;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_miscellaneous_options {
    m_flags: u8,
    m_round_time_limit_minutes: u8,
    m_round_limit: u8,
    m_early_victory_win_count: u8,
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_respawn_options {
    m_flags: u8,
    m_lives_per_round: u8,
    m_team_lives_per_round: u8,
    m_respawn_time_seconds: u8,
    m_suicide_penalty_seconds: u8,
    m_betrayal_penalty_seconds: u8,
    m_unknown_penalty_seconds: u8,
    m_respawn_growth_seconds: u8,
    m_respawn_player_traits_duration_seconds: u8,
    pad: [u8; 3],
    m_respawn_player_traits: c_player_traits,
}