use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::bitstream::c_bitstream;
use crate::blam::common::math::real_math::{real_point3d, real_rectangle3d};
use crate::blam::halo_3::release::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::types::array::Array;
use blf_lib_derive::PackedSerialize;
use crate::blam::common::math::real_math::vector3d;

const k_object_type_count: usize = 14;
const k_number_of_map_variant_simulation_entities: usize = 80;

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
#[PackedSerialize(1, BigEndian)]
pub struct c_map_variant {
    m_metadata: s_content_item_metadata,
    m_map_variant_version: u16,
    m_number_of_scenario_objects: u16,
    m_number_of_variant_objects: u16,
    m_number_of_placeable_object_quotas: u16,
    m_map_id: u32,
    m_world_bounds: real_rectangle3d,
    m_game_engine_subtype: u32,
    m_maximum_budget: f32,
    m_spent_budget: f32,
    m_helpers_enabled: bool,
    m_built_in: bool,
    __pad12A: [u8; 2],
    m_map_variant_checksum: u32,
    m_variant_objects: Array<s_variant_object_datum, 640>,
    m_object_type_start_index: Array<i16, k_object_type_count>,
    m_quotas: Array<s_variant_quota, 256>,
    m_chunk_simulation_object_glue_indices: Array<u32, k_number_of_map_variant_simulation_entities>,
}

impl c_map_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream) {
        self.m_metadata.encode(bitstream);
    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy, PackedSerialize, Serialize, Deserialize)]
#[PackedSerialize(1, BigEndian)]
pub struct s_variant_quota {
    object_definition_index: i32,
    minimum_count: u8,
    maximum_count: u8,
    placed_on_map: u8,
    maximum_allowed: i8,
    price_per_item: f32,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, PackedSerialize, Serialize, Deserialize)]
#[PackedSerialize(1, BigEndian)]
pub struct s_variant_multiplayer_object_properties_definition {
    game_engine_flags: u16,
    symmetry_placement_flags: u8, // foo
    owner_team: i8, // byte?
    shared_storage: u8, // spare_clips, teleporter_channel, spawn_rate
    spawn_time: u8,
    object_type: i8,
    boundary_shape: u8,
    boundary_size: f32, // width or radius
    boundary_box_length: f32,
    boundary_positive_height: f32,
    boundary_negative_height: f32,
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Copy, Serialize, Deserialize)]
#[PackedSerialize(4, BigEndian)]
pub struct s_variant_object_datum {
    flags: u16,
    // pad 16
    object_datum_index: i32,
    editor_object_index: i32,
    variant_quota_index: i32,
    position: real_point3d,
    forward: vector3d,
    up: vector3d,
    parent_object_identifier: c_object_identifier,
    multiplayer_game_object_properties: s_variant_multiplayer_object_properties_definition,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, PackedSerialize, Serialize, Deserialize)]
#[PackedSerialize(1, BigEndian)]
pub struct c_object_identifier {
    m_unique_id: i32,
    m_origin_bsp_index: i16,
    m_type: i8,
    m_source: i8,
}