use serde::{Deserialize, Serialize};
use blf_lib::blam::math::real_math::real_point3d;
use blf_lib_derive::PackedSerialize;
use crate::blam::math::real_math::vector3d;

#[derive(Default, PartialEq, Debug, Clone, Copy, PackedSerialize, Serialize, Deserialize)]
pub struct s_variant_multiplayer_object_properties_definition {
    symmetry_placement_flags: u16,
    game_engine_flags: u16,
    owner_team: u16,
    shared_storage: u8, // spare_clips, teleporter_channel, spawn_order
    spawn_rate: u8,
    teleporter_channel: u8,
    boundary_shape: u8,
    boundary_size: f32, // width or radius
    boundary_box_length: f32,
    boundary_positive_height: f32,
    boundary_negative_height: f32,
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Copy, Serialize, Deserialize)]
pub struct s_variant_object_datum {
    flags: u16,
    // pad 16
    object_datum_index: u32,
    editor_object_index: u32,
    variant_quota_index: u32,
    position: real_point3d,
    forward: vector3d,
    up: vector3d,
    parent_object_identifier: c_object_identifier,
    multiplayer_game_object_properties: s_variant_multiplayer_object_properties_definition,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, PackedSerialize, Serialize, Deserialize)]
pub struct c_object_identifier {
    m_unique_id: u32,
    m_origin_bsp_index: u16,
    m_type: u8,
    m_source: u8,
}