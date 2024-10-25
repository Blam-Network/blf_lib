use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::blam::halo_3::release::game::game_engine_traits::{c_game_engine_miscellaneous_options, c_game_engine_respawn_options};
use blf_lib::types::byte_limited_utf8_string::ByteLimitedUTF8String;
use blf_lib_derive::PackedSerialize;
use crate::blam::halo_3::release::saved_games::saved_game_files::s_content_item_metadata;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_social_options {
    m_flags: u16,
    m_team_changing: u16,
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_map_override_options {
    m_flags: u32,
    m_base_player_traits: c_player_traits,
    m_weapon_set_absolute_index: i16,
    m_vehicle_set_absolute_index: i16,
    m_red_powerup_traits: c_player_traits,
    m_blue_powerup_traits: c_player_traits,
    m_yellow_powerup_traits: c_player_traits,
    m_red_powerup_duration_seconds: u8,
    m_blue_powerup_duration_seconds: u8,
    m_yellow_powerup_duration_seconds: u8,
    pad: u8, // john hold on
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_game_engine_base_variant {
    m_checksum: u32,
    m_name: ByteLimitedUTF8String<32>,
    m_metadata: s_content_item_metadata,
    m_miscellaneous_options: c_game_engine_miscellaneous_options,
    m_respawn_options: c_game_engine_respawn_options,
    m_social_options: c_game_engine_social_options,
    m_map_override_options: c_game_engine_map_override_options,
    m_flags: u16,
    m_team_scoring_method: u16,
    // hold on john
}