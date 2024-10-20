use bincode::config::BigEndian;
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::bitstream::c_bitstream;
use blf_lib::TEST_BIT;
use crate::blam::common::math::real_math::{real_point3d, real_rectangle3d};
use crate::blam::halo_3::release::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::types::array::Array;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::PACK1;
use blf_lib_derive::PackedSerialize;
use crate::blam::common::math::real_math::vector3d;
use crate::blam::common::simulation::simulation_encoding::simulation_write_quantized_position;
use crate::io::packed_encoding::PackedEncoder;

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
        bitstream.write_integer(self.m_map_variant_version as u32, 8);
        bitstream.write_integer(self.m_map_variant_checksum, 32);
        bitstream.write_integer(self.m_number_of_scenario_objects as u32, 10);
        bitstream.write_integer(self.m_number_of_variant_objects as u32, 10);
        bitstream.write_integer(self.m_number_of_placeable_object_quotas as u32, 9);
        bitstream.write_integer(self.m_map_id, 32);
        bitstream.write_integer(if self.m_built_in { 1 } else { 0 }, 1);
        bitstream.write_raw_data(self.m_world_bounds.encode_packed(Endianness::Big, PACK1).as_slice(), 0xC0);
        bitstream.write_integer(self.m_game_engine_subtype, 4);
        bitstream.write_float(self.m_maximum_budget, 32);
        bitstream.write_float(self.m_spent_budget, 32);

        for i in 0..self.m_number_of_variant_objects as usize {
            let variant_object = self.m_variant_objects[i];

            if variant_object.flags & 0x3FF == 0 // 0x3FF is 10 bits, there's 10 flags. If none are set...
            {
                bitstream.write_integer(0, 1); // variant_object_exists
            }
            else
            {
                bitstream.write_integer(1, 1); // variant_object_exists
                bitstream.write_integer(variant_object.flags as u32, 16);
                bitstream.write_integer(variant_object.variant_quota_index as u32, 32);

                if TEST_BIT!(variant_object.flags, 8) // spawns relative
                {
                    bitstream.write_integer(1, 1); // parent-object-exists
                    bitstream.write_raw_data(variant_object.parent_object_identifier.encode_packed(Endianness::Big, PACK1).as_slice(), 64);
                }
                else
                {
                    bitstream.write_integer(0, 1); // parent-object-exists
                }

                if !TEST_BIT!(variant_object.flags, 1) && i < self.m_number_of_scenario_objects as usize  //edited
                {
                    bitstream.write_integer(0, 1);
                }
                else
                {
                    bitstream.write_integer(1, 1);
                    simulation_write_quantized_position(bitstream, &variant_object.position, 16, false, &self.m_world_bounds);
                    bitstream.write_axes(&variant_object.up, &variant_object.forward);
                    bitstream.write_axes(&variant_object.forward, &variant_object.up);
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.object_type as u32, 8);
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.symmetry_placement_flags as u32, 8);
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.game_engine_flags as u32, 16);
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.shared_storage as u32, 8);
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.spawn_time as u32, 8);
                    bitstream.write_signed_integer(variant_object.multiplayer_game_object_properties.owner_team as i32, 8);
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_shape as u32, 8);

                    match variant_object.multiplayer_game_object_properties.boundary_shape {
                        1 => { // sphere
                            bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_size as u32, 16);
                            bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_negative_height as u32, 16);
                        }
                        2 => { // cylinder
                            bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_size as u32, 16);
                            bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_box_length as u32, 16);
                            bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_positive_height as u32, 16);
                        }
                        3 => { // box
                            bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_size as u32, 16);
                            bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_box_length as u32, 16);
                            bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_positive_height as u32, 16);
                            bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_negative_height as u32, 16);
                        }
                        _ => { }
                    }
                }
            }
        }

        for i in 0..k_object_type_count {
            bitstream.write_signed_integer(self.m_object_type_start_index[i] as i32 + 1, 9);
        }

        for i in 0..self.m_number_of_placeable_object_quotas as usize {
            let object_quota = self.m_quotas[i];
            bitstream.write_integer(object_quota.object_definition_index as u32, 32);
            bitstream.write_integer(object_quota.minimum_count as u32, 8);
            bitstream.write_integer(object_quota.maximum_count as u32, 8);
            bitstream.write_integer(object_quota.placed_on_map as u32, 8);
            bitstream.write_integer(object_quota.maximum_allowed as u32, 8);
            bitstream.write_float(object_quota.price_per_item, 32);
        }

    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy, PackedSerialize, Serialize, Deserialize)]
#[PackedSerialize(1, BigEndian)]
pub struct s_variant_quota {
    pub object_definition_index: i32,
    pub minimum_count: u8,
    pub maximum_count: u8,
    pub placed_on_map: u8,
    pub maximum_allowed: i8,
    pub price_per_item: f32,
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