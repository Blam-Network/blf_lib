use std::io::Cursor;
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::io::packed_decoding::PackedDecoder;
use blf_lib::TEST_BIT;
use crate::blam::common::math::real_math::{real_point3d, real_rectangle3d};
use crate::blam::halo_3::release::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::types::array::Array;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::PACK1;
use blf_lib_derive::PackedSerialize;
use crate::blam::common::math::real_math::vector3d;
use crate::blam::common::simulation::simulation_encoding::{simulation_read_quantized_position, simulation_write_quantized_position};
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
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        self.m_metadata.encode(bitstream);
        bitstream.write_integer(self.m_map_variant_version as u32, 8);
        bitstream.write_integer(self.m_map_variant_checksum, 32);
        bitstream.write_integer(self.m_number_of_scenario_objects as u32, 10);
        bitstream.write_integer(self.m_number_of_variant_objects as u32, 10);
        bitstream.write_integer(self.m_number_of_placeable_object_quotas as u32, 9);
        bitstream.write_integer(self.m_map_id, 32);
        bitstream.write_bool(self.m_built_in);
        bitstream.write_raw_data(self.m_world_bounds.encode_packed(Endianness::Big, PACK1).as_slice(), 0xC0);
        bitstream.write_integer(self.m_game_engine_subtype, 4);
        bitstream.write_float(self.m_maximum_budget, 32);
        bitstream.write_float(self.m_spent_budget, 32);

        for i in 0..self.m_number_of_variant_objects as usize {
            let variant_object = self.m_variant_objects[i];

            if variant_object.flags & 0x3FF == 0 // 0x3FF is 10 bits, there's 10 flags. If none are set...
            {
                bitstream.write_bool(false); // variant_object_exists
            }
            else
            {
                bitstream.write_bool(true); // variant_object_exists
                bitstream.write_integer(variant_object.flags as u32, 16);
                bitstream.write_integer(variant_object.variant_quota_index as u32, 32);

                if TEST_BIT!(variant_object.flags, 8) // spawns relative
                {
                    bitstream.write_bool(true); // parent-object-exists
                    bitstream.write_raw_data(variant_object.parent_object_identifier.encode_packed(Endianness::Big, PACK1).as_slice(), 64);
                }
                else
                {
                    bitstream.write_bool(false); // parent-object-exists
                }

                if !TEST_BIT!(variant_object.flags, 1) && i < self.m_number_of_scenario_objects as usize  //edited
                {
                    bitstream.write_bool(false);
                }
                else
                {
                    bitstream.write_bool(true);
                    simulation_write_quantized_position(bitstream, &variant_object.position, 16, false, &self.m_world_bounds);
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

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        self.m_metadata.decode(bitstream);
        self.m_map_variant_version = bitstream.read_u8(8) as u16;
        self.m_map_variant_checksum = bitstream.read_integer(32);
        self.m_number_of_scenario_objects = bitstream.read_u16(10);
        self.m_number_of_variant_objects = bitstream.read_u16(10);
        self.m_number_of_placeable_object_quotas = bitstream.read_u16(9);
        self.m_map_id = bitstream.read_integer(32);
        self.m_built_in = bitstream.read_bool();
        self.m_world_bounds = real_rectangle3d::decode_packed(&mut Cursor::new(&bitstream.read_raw_data(0xC0)), Endianness::Big, PACK1).unwrap();
        self.m_game_engine_subtype = bitstream.read_u8(4) as u32;
        self.m_maximum_budget = bitstream.read_float(32);
        self.m_spent_budget = bitstream.read_float(32);

        for i in 0..self.m_number_of_variant_objects as usize {
            let variant_object = &mut self.m_variant_objects.get_mut()[i];

            let variant_object_exists = bitstream.read_bool();

            if !variant_object_exists {
                continue;
            }

            variant_object.flags = bitstream.read_u16(16);
            variant_object.variant_quota_index = bitstream.read_signed_integer(32);

            let parent_object_exists = bitstream.read_bool();
            if parent_object_exists {
                variant_object.parent_object_identifier = c_object_identifier::decode_packed(&mut Cursor::new(&bitstream.read_raw_data(64)), Endianness::Big, PACK1).unwrap();
            }

            let position_exists = bitstream.read_bool();
            if position_exists {
                simulation_read_quantized_position(bitstream, &mut variant_object.position, 16, &self.m_world_bounds);
                bitstream.read_axis(&mut variant_object.forward, &mut variant_object.up);
                variant_object.multiplayer_game_object_properties.object_type = bitstream.read_u8(8) as i8;
                variant_object.multiplayer_game_object_properties.symmetry_placement_flags = bitstream.read_u8(8);
                variant_object.multiplayer_game_object_properties.game_engine_flags = bitstream.read_u16(16);
                variant_object.multiplayer_game_object_properties.shared_storage = bitstream.read_u8(8);
                variant_object.multiplayer_game_object_properties.spawn_time = bitstream.read_u8(8);
                variant_object.multiplayer_game_object_properties.owner_team = bitstream.read_u8(8) as i8;
                variant_object.multiplayer_game_object_properties.boundary_shape = bitstream.read_u8(8);

                match variant_object.multiplayer_game_object_properties.boundary_shape {
                    1 => { // sphere
                        variant_object.multiplayer_game_object_properties.boundary_size = bitstream.read_u16(16) as f32;
                        variant_object.multiplayer_game_object_properties.boundary_negative_height = bitstream.read_u16(16) as f32;
                    }
                    2 => { // cylinder
                        variant_object.multiplayer_game_object_properties.boundary_size = bitstream.read_u16(16) as f32;
                        variant_object.multiplayer_game_object_properties.boundary_box_length = bitstream.read_u16(16) as f32;
                        variant_object.multiplayer_game_object_properties.boundary_positive_height = bitstream.read_u16(16) as f32;
                    }
                    3 => { // box
                        variant_object.multiplayer_game_object_properties.boundary_size = bitstream.read_u16(16) as f32;
                        variant_object.multiplayer_game_object_properties.boundary_box_length = bitstream.read_u16(16) as f32;
                        variant_object.multiplayer_game_object_properties.boundary_positive_height = bitstream.read_u16(16) as f32;
                        variant_object.multiplayer_game_object_properties.boundary_negative_height = bitstream.read_u16(16) as f32;
                    }
                    _ => { }
                }

            }
        }

        for i in 0..k_object_type_count {
            self.m_object_type_start_index.get_mut()[i] = bitstream.read_u16(9) as i16 - 1;
        }

        for i in 0..self.m_number_of_placeable_object_quotas as usize {
            let object_quota = &mut self.m_quotas.get_mut()[i];
            object_quota.object_definition_index = bitstream.read_signed_integer(32);
            object_quota.minimum_count = bitstream.read_u8(8);
            object_quota.maximum_count = bitstream.read_u8(8);
            object_quota.placed_on_map = bitstream.read_u8(8);
            object_quota.maximum_allowed = bitstream.read_u8(8) as i8;
            object_quota.price_per_item = bitstream.read_float(32);
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