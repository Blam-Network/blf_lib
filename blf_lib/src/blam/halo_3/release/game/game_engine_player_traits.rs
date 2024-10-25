use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_player_trait_weapons {
    m_initial_grenade_count_setting: u16,
    m_initial_primary_weapon_absolute_index: i8,
    m_initial_secondary_weapon_absolute_index: i8,
    m_damage_modifier_percentage_setting: u8,
    m_recharging_grenades_setting: u8,
    m_infinite_ammo_setting: u8,
    m_weapon_pickup_setting: u8,
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_player_trait_shield_vitality {
    m_damage_resistance_percentage_setting: u8,
    m_shield_recharge_rate_percentage_setting: u8,
    m_vampirism_percentage_setting: u8,
    m_headshot_immunity_setting: u8,
    m_shield_multiplier_setting: u8,
    pad: [u8;3], // pelican
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_player_trait_movement {
    m_speed_setting: u8,
    m_gravity_setting: u8,
    m_vehicle_usage_setting: u8,
    pad: u8, // hornet
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_player_trait_appearance {
    m_active_camo_setting: u8,
    m_waypoint_setting: u8,
    m_aura_setting: u8,
    m_forced_change_color_setting: u8,
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_player_trait_sensors {
    m_motion_tracker_setting: u16,
    m_motion_tracker_range_setting: u16,
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct c_player_traits {
    m_shield_vitality_traits: c_player_trait_shield_vitality,
    m_weapon_traits: c_player_trait_weapons,
    m_movement_traits: c_player_trait_movement,
    m_appearance_traits: c_player_trait_appearance,
    m_sensor_traits: c_player_trait_sensors,
}