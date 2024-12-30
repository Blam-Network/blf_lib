use std::io::{Read, Seek, Write};
use std::net::Ipv4Addr;
use std::u32;
use binrw::{binrw, BinRead, BinReaderExt, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::{BlfChunk, TestSize};
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::types::array::StaticArray;
use blf_lib::types::bool::s_bool;
use blf_lib::types::c_string::StaticString;
use blf_lib::types::time::{time32_t, time64_t};

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite)]
#[Header("mqdt", 1.5)]
#[brw(big)]
#[Size(0x9874)]
pub struct s_blf_chunk_matchmaking_quality_data
{
    #[brw(pad_before = 4)]
    pub data: s_matchmaking_quality_data,
}
#[derive(Default,PartialEq,Debug,Clone,BinRead,BinWrite,Serialize,Deserialize,TestSize)]
#[brw(big)]
#[Size(0x9870)]
pub struct s_matchmaking_quality_data {
    pub aborted: s_bool,
    pub assemble_timed_out: s_bool,
    pub left_assemble: s_bool,
    pub left_arbitration: s_bool,
    pub not_enough_hosts: s_bool,
    pub left_host_selection: s_bool,
    pub left_prepare_map: s_bool,
    pub vetoed: s_bool,
    pub hit_arbitration_waiting_for_establishment_and_connectivity_give_up_time: s_bool,
    pub hit_arbitration_waiting_for_completion_give_up_time: s_bool,
    pub searching: s_bool,
    pub gathering: s_bool,
    pub gathering_by_force: s_bool,
    #[brw(pad_before = 3)]
    pub vetoed_or_played_game: s_game_hopper_picked_game,
    pub ping_msec: i32,
    pub search_time: u32,
    pub gather_time: u32,
    pub arbitration_time: u32,
    pub host_selection_time: u32,
    pub prepare_map_time: u32,
    pub in_match_time: u32,
    pub local_address: s_matchmaking_quality_address,
    pub qos_sample_count: i32,
    pub qos_samples: StaticArray<s_matchmaking_quality_qos_sample, 400>, // probably fine
    pub session_search_count: i32,
    pub session_searches: StaticArray<s_matchmaking_session_search, 16>,
    pub live_service_qos_result_valid: s_bool,
    #[brw(pad_before = 3)]
    pub live_service_qos_result: s_transport_qos_result,
    pub nat_type_valid: s_bool,
    #[brw(pad_before = 3)]
    pub nat_type: e_online_nat_type,
    pub primary_map_load_failure: s_bool,
    #[brw(pad_after = 2)]
    pub secondary_map_load_failure: s_bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[brw(big)]
#[Size(0x20)]
pub struct s_matchmaking_quality_address {
    #[br(map = |x: u32| Ipv4Addr::from(x))]
    #[bw(try_map = |addr: &Ipv4Addr| -> Result<u32, binrw::Error> { Ok(u32::from(*addr)) })]
    pub ina: Ipv4Addr,
    #[br(map = |x: u32| Ipv4Addr::from(x))]
    #[bw(try_map = |addr: &Ipv4Addr| -> Result<u32, binrw::Error> { Ok(u32::from(*addr)) })]
    pub ina_online: Ipv4Addr,
    #[brw(pad_after = 2)]
    pub w_port_online: u16,
    pub ab_online: [u8; 20],
}

impl Default for s_matchmaking_quality_address {
    fn default() -> Self {
        Self {
            ina: Ipv4Addr::new(0, 0, 0, 0),
            ina_online: Ipv4Addr::new(0, 0, 0, 0),
            w_port_online: 0,
            ab_online: [0; 20],
        }
    }
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[brw(big)]
#[Size(0x20)]
pub struct s_transport_qos_result {
    pub probes_sent: i32,
    pub probes_received: i32,
    pub ping_msec_minimum: i32,
    pub ping_msec_median: i32,
    pub bandwidth_upstream_bps: i32,
    pub bandwidth_downstream_bps: i32,
    pub data_block_size: i32,
    pub data_block: u32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0x6C)]
#[brw(big)]
pub struct s_game_hopper_picked_game {
    pub map_id: i32,
    pub game_variant_name: StaticString<32>,
    pub game_variant_hash: s_network_http_request_hash,
    pub map_variant_name: StaticString<32>,
    pub map_variant_hash: s_network_http_request_hash,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0x40)]
#[brw(big)]
pub struct s_matchmaking_quality_qos_sample {
    pub address: s_matchmaking_quality_address,
    pub qos_result: s_transport_qos_result,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[brw(big)]
#[Size(0xB8)]
pub struct s_matchmaking_session_search_status {
    pub session_count: i32,
    pub start_timestamp: u32,
    pub completed_timestamp: u32,
    pub failure_count: i32,
    pub retry_count: i32,
    pub last_failure_time: u32,
    pub stage: e_matchmaking_search_stage,
    pub qos_desired_count: i32,
    pub probe_only_qos_completed_count: i32,
    pub full_qos_completed_count: i32,
    pub unsuitable_session_count: i32,
    pub unsuitable_reasons_count: StaticArray<i32, 12>,
    pub undesireable_session_count: i32,
    pub undesirable_reasons_count: StaticArray<i32, 2>,
    pub join_results: StaticArray<i32, 20>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[brw(big)]
#[Size(0x54)]
pub struct s_matchmaking_session_search_query {
    pub stage: e_matchmaking_search_stage,
    pub stage_round: i32,
    pub hopper_identifier: u16,
    #[brw(pad_after = 2)]
    pub min_skill_level: i32,
    pub max_skill_level: i32,
    pub party_size: i32,
    #[brw(pad_after = 3)]
    pub mixed_skill_party: s_bool,
    pub nat_type: e_online_nat_type,
    pub min_average_skill_level: i32,
    pub max_average_skill_level: i32,
    pub average_mu_min: f32,
    pub average_mu_max: f32,
    pub min_average_experience_rating: i32,
    pub max_average_experience_rating: i32,
    pub gamer_zone: e_gamer_zone,
    pub gamer_region: u32,
    pub language: e_language,
    pub connection_threshold_ms: i32,
    #[brw(pad_after = 3)]
    pub session_of_quitters: s_bool,
    pub search_preference: e_matchmaking_search_preference,
    #[brw(pad_after = 2)]
    pub query_flags: u16,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[brw(big)]
#[Size(0x338)]
pub struct s_matchmaking_session_search {
    pub status: s_matchmaking_session_search_status,
    pub query: s_matchmaking_session_search_query,
    #[brw(pad_before = 4)]
    pub online_query: s_online_session_search_parameters,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_matchmaking_search_stage {
    #[default]
    _matchmaking_search_stage_strict_skill = 0,
    _matchmaking_search_stage_skill = 1,
    _matchmaking_search_stage_any = 2,
    _matchmaking_search_stage_desparation = 3,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[brw(big)]
#[Size(0x228)]
pub struct s_online_session_search_parameters {
    pub query: e_online_session_search_query_id,
    pub controller_index: u32,
    pub property_count: i32,
    #[brw(pad_before = 4)]
    pub properties: StaticArray<s_online_property, 20>,
    pub context_count: i32,
    #[brw(pad_after = 4)]
    pub contexts: StaticArray<s_online_context, 6>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[brw(big)]
#[Size(8)]
pub struct s_online_context {
    pub id: e_online_context_id,
    pub value: e_online_context_value,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_online_context_id {
    #[default]
    _online_context_id_game_mode = 0,
    _online_context_id_game_type = 1,
    _online_context_id_non_matchmade_game_mode = 2,
    _online_context_id_game_size = 3,
    _online_context_id_map_size = 4,
    _online_context_game_rules = 5,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_online_context_value {
    #[default]
    _online_context_value_default = 0,
    _online_context_value_true = 1,
    _online_context_value_false = 2,
    _online_context_value_game_type_standard = 3,
    _online_context_value_game_type_ranked = 4,
    _online_context_value_game_size_small = 5,
    _online_context_value_game_size_medium = 6,
    _online_context_value_game_size_large = 7,
    _online_context_value_map_size_small = 8,
    _online_context_value_map_size_medium = 9,
    _online_context_value_map_size_large = 10,
    _online_context_value_game_rules_unknown = 11,
    _online_context_value_game_rules_ctf = 12,
    _online_context_value_game_rules_slayer = 13,
    _online_context_value_game_rules_oddball = 14,
    _online_context_value_game_rules_king = 15,
    _online_context_value_game_rules_sandbox = 16,
    _online_context_value_game_rules_vip = 17,
    _online_context_value_game_rules_juggernaut = 18,
    _online_context_value_game_rules_territories = 19,
    _online_context_value_game_rules_assault = 20,
    _online_context_value_game_rules_infection = 21,
    _online_context_value_game_mode_non_matchmaking = 22,
    _online_context_value_game_mode_hopper_0 = 23,
    _online_context_value_game_mode_hopper_1 = 24,
    _online_context_value_game_mode_hopper_2 = 25,
    _online_context_value_game_mode_hopper_3 = 26,
    _online_context_value_game_mode_hopper_4 = 27,
    _online_context_value_game_mode_hopper_5 = 28,
    _online_context_value_game_mode_hopper_6 = 29,
    _online_context_value_game_mode_hopper_7 = 30,
    _online_context_value_game_mode_hopper_8 = 31,
    _online_context_value_game_mode_hopper_9 = 32,
    _online_context_value_game_mode_hopper_10 = 33,
    _online_context_value_game_mode_hopper_11 = 34,
    _online_context_value_game_mode_hopper_12 = 35,
    _online_context_value_game_mode_hopper_13 = 36,
    _online_context_value_game_mode_hopper_14 = 37,
    _online_context_value_game_mode_hopper_15 = 38,
    _online_context_value_game_mode_hopper_16 = 39,
    _online_context_value_game_mode_hopper_17 = 40,
    _online_context_value_game_mode_hopper_18 = 41,
    _online_context_value_game_mode_hopper_19 = 42,
    _online_context_value_game_mode_hopper_20 = 43,
    _online_context_value_game_mode_hopper_21 = 44,
    _online_context_value_game_mode_hopper_22 = 45,
    _online_context_value_game_mode_hopper_23 = 46,
    _online_context_value_game_mode_hopper_24 = 47,
    _online_context_value_game_mode_hopper_25 = 48,
    _online_context_value_game_mode_hopper_26 = 49,
    _online_context_value_game_mode_hopper_27 = 50,
    _online_context_value_game_mode_hopper_28 = 51,
    _online_context_value_game_mode_hopper_29 = 52,
    _online_context_value_game_mode_hopper_30 = 53,
    _online_context_value_game_mode_hopper_31 = 54,
    _online_context_value_game_mode_custom = 55,
    _online_context_value_game_mode_campaign = 56,
    _online_context_value_game_mode_map_editor = 57,
    _online_context_value_game_mode_theater = 58,
}



#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_online_session_search_query_id {
    #[default]
    _online_session_search_query_id_playlist = 0,
    _online_session_search_query_id_recruiting = 1,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[brw(big)]
#[Size(0x18)]
pub struct s_online_property {
    pub id: e_online_property_id,
    #[brw(pad_before = 4)]
    pub value: s_online_data,
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_online_property_id {
    #[default]
    _online_property_id_gamer_language = 0,
    _online_property_id_gamer_zone = 1,
    _online_property_id_gamer_hostname = 2,
    _online_property_id_affiliate_score = 3,
    _online_property_id_skill_draw_probability = 4,
    _online_property_id_skill_beta = 5,
    _online_property_id_skill_tau = 6,
    _online_property_id_relative_score = 7,
    _online_property_id_session_team = 8,
    _online_property_id_skill_hill_climbing_factor = 9,
    _online_property_id_hopper_skill = 10,
    _online_property_id_hopper_games_played = 11,
    _online_property_id_hopper_games_completed = 12,
    _online_property_id_hopper_games_won = 13,
    _online_property_id_hopper_experience_base = 14,
    _online_property_id_hopper_experience_penalty = 15,
    _online_property_id_global_experience_base = 16,
    _online_property_id_global_experience_penalty = 17,
    _online_property_id_global_highest_skill_level_attained = 18,
    _online_property_id_global_matchmade_ranked_games_played = 19,
    _online_property_id_global_matchmade_ranked_games_completed = 20,
    _online_property_id_global_matchmade_ranked_games_won = 21,
    _online_property_id_global_matchmade_unranked_games_played = 22,
    _online_property_id_global_matchmade_unranked_games_completed = 23,
    _online_property_id_global_matchmade_unranked_games_won = 24,
    _online_property_id_global_custom_games_played = 25,
    _online_property_id_global_custom_games_completed = 26,
    _online_property_id_global_custom_games_won = 27,
    _online_property_id_global_first_game_played_date = 28,
    _online_property_id_global_last_game_played_date = 29,
    _online_property_id_hopper_identifier = 30,
    _online_property_id_lowest_skill_level = 31,
    _online_property_id_highest_skill_level = 32,
    _online_property_id_average_skill_level = 33,
    _online_property_id_average_experience_rating = 34,
    _online_property_id_has_all_dlc_maps = 35,
    _online_property_id_session_of_quitters = 36,
    _online_property_id_language_override = 37,
    _online_property_id_gamerzone_override = 38,
    _online_property_id_country_code_override = 39,
    _online_property_id_mixed_skill_restricted = 40,
    _online_property_id_joinable_party_size_1 = 41,
    _online_property_id_joinable_party_size_2 = 42,
    _online_property_id_joinable_party_size_3 = 43,
    _online_property_id_joinable_party_size_4 = 44,
    _online_property_id_joinable_party_size_5 = 45,
    _online_property_id_joinable_party_size_6 = 46,
    _online_property_id_joinable_party_size_7 = 47,
    _online_property_id_joinable_party_size_8 = 48,
    _online_property_id_joinable_party_size_9 = 49,
    _online_property_id_nat_type = 50,
    _online_property_id_average_mu = 51,
    _online_property_id_min_average_skill_level = 52,
    _online_property_id_max_average_skill_level = 53,
    _online_property_id_min_average_experience_rating = 54,
    _online_property_id_max_average_experience_rating = 55,
    _online_property_id_min_average_mu = 56,
    _online_property_id_max_average_mu = 57,
    _online_property_id_min_skill_level = 58,
    _online_property_id_max_skill_level = 59,
    _online_property_unused = 60,
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u8)]
pub enum e_online_data_type {
    #[default]
    DataTypeUnknown = 0,
    DataTypeLong = 1,
    DataTypeQword = 2,
    DataTypeDouble = 3,
    DataTypeString = 4,
    DataTypeFloat = 5,
    DataTypeBinary = 6,
    DataTypeDayTime = 7,
}


#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, TestSize)]
#[Size(0x10)]
pub struct s_online_data {
    pub data_type: e_online_data_type,
    pub u32_value: u32,
    pub u64_value: u64,
    pub f64_value: f64,
    pub string_value: (u32, u32), // size, ptr
    pub binary_value: (u32, u32), // size, ptr
    pub f32_value: f32,
    pub time64_value: time64_t,
}

impl BinWrite for s_online_data {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        match endian {
            Endian::Big => {
                writer.write_be(&self.data_type)?;
                writer.write_be(&0u32)?;
                writer.write_be(&0u16)?;
                writer.write_be(&0u8)?;

                match self.data_type {
                    e_online_data_type::DataTypeUnknown => {
                        writer.write_be(&0u64)?;
                        Ok(())
                    }
                    e_online_data_type::DataTypeBinary => {
                        writer.write_be(&self.binary_value.0)?;
                        writer.write_be(&self.binary_value.1)
                    }
                    e_online_data_type::DataTypeDouble => { writer.write_be(&self.f64_value) }
                    e_online_data_type::DataTypeFloat => {
                        writer.write_be(&self.f32_value)?;
                        writer.write_be(&0u32)?;
                        Ok(())
                    }
                    e_online_data_type::DataTypeLong => {
                        writer.write_be(&self.u32_value)?;
                        writer.write_be(&0u32)?;
                        Ok(())
                    }
                    e_online_data_type::DataTypeQword => { writer.write_be(&self.u64_value) }
                    e_online_data_type::DataTypeString => {
                        writer.write_be(&self.string_value.0)?;
                        writer.write_be(&self.string_value.1)
                    }
                    e_online_data_type::DataTypeDayTime => { writer.write_be(&self.time64_value) }
                }
            }
            Endian::Little => {
                writer.write_le(&self.data_type)?;
                writer.write_le(&0u32)?;
                writer.write_le(&0u16)?;
                writer.write_le(&0u8)?;

                match self.data_type {
                    e_online_data_type::DataTypeUnknown => {
                        writer.write_le(&0u64)?;
                        Ok(())
                    }
                    e_online_data_type::DataTypeString => {
                        writer.write_le(&self.string_value.0)?;
                        writer.write_le(&self.string_value.1)
                    }                    e_online_data_type::DataTypeDouble => { writer.write_le(&self.f64_value) }
                    e_online_data_type::DataTypeFloat => {
                        writer.write_le(&self.f32_value)?;
                        writer.write_le(&0u32)?;
                        Ok(())
                    }
                    e_online_data_type::DataTypeLong => {
                        writer.write_le(&self.u32_value)?;
                        writer.write_le(&0u32)?;
                        Ok(())
                    }
                    e_online_data_type::DataTypeQword => { writer.write_le(&self.u64_value) }
                    e_online_data_type::DataTypeBinary => {
                        writer.write_le(&self.binary_value.0)?;
                        writer.write_le(&self.binary_value.1)
                    }
                    e_online_data_type::DataTypeDayTime => { writer.write_le(&self.time64_value) }
                }
            }
        }
    }
}

impl BinRead for s_online_data {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        match endian {
            Endian::Big => {
                let mut online_property = s_online_data {
                    data_type: reader.read_be()?,
                    u32_value: 0,
                    u64_value: 0,
                    f64_value: 0.0,
                    binary_value: (0, 0),
                    string_value: (0, 0),
                    f32_value: 0.0,
                    time64_value: Default::default(),
                };

                reader.seek_relative(7)?;

                match online_property.data_type {
                    e_online_data_type::DataTypeUnknown => {
                        reader.seek_relative(8)?;
                    }
                    e_online_data_type::DataTypeFloat => {
                        online_property.f32_value = reader.read_be()?;
                        reader.seek_relative(4)?;
                    }
                    e_online_data_type::DataTypeDouble => { online_property.f64_value = reader.read_be()?; }
                    e_online_data_type::DataTypeString => {
                        online_property.string_value.0 = reader.read_be()?;
                        online_property.string_value.1 = reader.read_be()?;
                    }
                    e_online_data_type::DataTypeQword => { online_property.u64_value = reader.read_be()?; }
                    e_online_data_type::DataTypeBinary => {
                        online_property.binary_value.0 = reader.read_be()?;
                        online_property.binary_value.1 = reader.read_be()?;
                    }
                    e_online_data_type::DataTypeDayTime => { online_property.time64_value = reader.read_be()?; }
                    e_online_data_type::DataTypeLong => {
                        online_property.u32_value = reader.read_be()?;
                        reader.seek_relative(4)?;
                    }
                }

                Ok(online_property)
            }
            Endian::Little => {
                let mut online_property = s_online_data {
                    data_type: reader.read_le()?,
                    u32_value: 0,
                    u64_value: 0,
                    f64_value: 0.0,
                    string_value: (0, 0),
                    binary_value: (0, 0),
                    f32_value: 0.0,
                    time64_value: Default::default(),
                };

                reader.seek_relative(7)?;

                match online_property.data_type {
                    e_online_data_type::DataTypeUnknown => {
                        reader.seek_relative(8)?;
                    }
                    e_online_data_type::DataTypeFloat => {
                        online_property.f32_value = reader.read_le()?;
                        reader.seek_relative(4)?;
                    }
                    e_online_data_type::DataTypeDouble => { online_property.f64_value = reader.read_le()?; }
                    e_online_data_type::DataTypeString => {
                        online_property.string_value.0 = reader.read_le()?;
                        online_property.string_value.1 = reader.read_le()?;
                    }
                    e_online_data_type::DataTypeQword => { online_property.u64_value = reader.read_le()?; }
                    e_online_data_type::DataTypeBinary => {
                        online_property.binary_value.0 = reader.read_le()?;
                        online_property.binary_value.1 = reader.read_le()?;
                    }
                    e_online_data_type::DataTypeDayTime => { online_property.time64_value = reader.read_le()?; }
                    e_online_data_type::DataTypeLong => {
                        online_property.u32_value = reader.read_le()?;
                        reader.seek_relative(4)?;
                    }
                }

                Ok(online_property)
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_online_nat_type {
    #[default]
    _online_nat_type_unknown = 0,
    _online_nat_type_open = 1,
    _online_nat_type_moderate = 2,
    _online_nat_type_strict = 3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_gamer_zone {
    #[default]
    _gamer_zone_xbox1 = 0,
    _gamer_zone_rr = 1,
    _gamer_zone_pro = 2,
    _gamer_zone_family = 3,
    _gamer_zone_underground = 4,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_language {
    #[default]
    _language_english = 0,
    _language_japanese = 1,
    _language_german = 2,
    _language_french = 3,
    _language_spanish = 4,
    _language_mexican_spanish = 5,
    _language_italian = 6,
    _language_korean = 7,
    _language_traditional_chinese = 8,
    _language_simplified_chinese = 9,
    _language_portuguese = 10,
    _language_polish = 11,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_matchmaking_search_preference {
    #[default]
    _matchmaking_search_preference_none = 0,
    _matchmaking_search_preference_skill = 1,
    _matchmaking_search_preference_good_connection = 2,
    _matchmaking_search_preference_language = 3,
}



impl BlfChunkHooks for s_blf_chunk_matchmaking_quality_data {}
