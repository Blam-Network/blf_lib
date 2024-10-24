use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_default::{c_game_engine_map_override_options, c_game_engine_social_options};
use blf_lib::blam::halo_3::release::game::game_engine_traits::{c_game_engine_miscellaneous_options, c_game_engine_respawn_options};
use blf_lib::blam::halo_3::release::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::types::byte_limited_utf8_string::ByteLimitedUTF8String;
use blf_lib_derive::PackedSerialize;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_slayer_variant {
    m_checksum: u32,
    m_name: ByteLimitedUTF8String<32>,
    m_metadata: s_content_item_metadata,
    m_miscellaneous_options: c_game_engine_miscellaneous_options,
    m_respawn_options: c_game_engine_respawn_options,
    m_social_options: c_game_engine_social_options,
    m_map_override_options: c_game_engine_map_override_options,
    m_flags: u16,
    m_team_scoring_method: u16,

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