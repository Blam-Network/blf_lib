use std::ffi::{c_char};
use std::u32;
use blf_lib::blam::math::real_math::real_rectangle3d;
use blf_lib::blam::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::blam::saved_games::scenario_map_variant::s_variant_object_datum;
use blf_lib::blf_chunk;
use blf_lib::types::array::Array;
use blf_lib::types::byte_limited_wchar_string::ByteLimitedWcharString;

blf_chunk!(
    #[Signature("mapv")]
    #[Version(12.0)]
    #[PackedSerialize(4, BigEndian)]
    pub struct s_blf_chunk_map_variant
    {
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
        // s_content_item_metadata m_metadata;
        //
        // short m_map_variant_version;
        //
        // short m_number_of_scenario_objects;
        // short m_number_of_variant_objects;
        // short m_number_of_placeable_object_quotas;
        //
        // long m_map_id;
        //
        // real_rectangle3d m_world_bounds;
        //
        // long m_game_engine_subtype;
        //
        // real m_maximum_budget;
        // real m_spent_budget;
        //
        // bool m_helpers_enabled;
        // bool m_built_in;
        // byte __pad12A[2];
        //
        // dword m_map_variant_checksum;
        //
        // c_static_array<s_variant_object_datum, 640> m_variant_objects;
        // c_static_array<short, k_object_type_count> m_object_type_start_index;
        // c_static_array<s_variant_quota, 256> m_quotas;
        // c_static_array<long, k_number_of_map_variant_simulation_entities> m_chunk_simulation_object_glue_indices;
        // byte unused[0xC4];
    }
);

impl s_blf_chunk_map_variant {
}