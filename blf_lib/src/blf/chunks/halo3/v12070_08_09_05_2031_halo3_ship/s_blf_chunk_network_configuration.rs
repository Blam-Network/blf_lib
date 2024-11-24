use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
use crate::types::c_string::StaticString;
use binrw::io::{Cursor, Seek};
use binrw::{binrw, BinRead, BinResult, BinWrite};
use binrw::BinReaderExt;
use crate::types::bool::s_bool;
use serde_hex::{SerHex,StrictCapPfx};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("netc", 135.1)]
#[Size(8300)]
#[brw(big)]
pub struct s_blf_chunk_network_configuration
{
    pub config: s_network_configuration,
}

impl BlfChunkHooks for s_blf_chunk_network_configuration {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_dlc_pack {
    #[default]
    dlc_pack_none = 0,
    dlc_pack_heroic = 1,
    dlc_pack_legendary = 2,
    dlc_pack_mythic = 3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_alpha_configuration_ui_level {
    alpha_ui_level_locked = 0,
    #[default]
    alpha_ui_level_main = 1,
    alpha_ui_level_internal_beta = 2,
    alpha_ui_level_external_beta = 3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_minidump_generation {
    minidump_generation_none = 0,
    minidump_generation_regular = 1,
    #[default]
    minidump_generation_minimini = 2,
    minidump_generation_count = 3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_map_status {
    #[default]
    map_status_default = 0,
    map_status_dlc = 1,
    map_status_replaced = 2,
    map_status_revoked = 3,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_network_file_download_configuration {
    pub master_catalog_file_download_interval_msec: i32,
    pub required_file_invalidation_check_interval_msec: i32,
    pub required_file_download_retry_interval_msec: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_bandwidth_configuration_host_preference_table {
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub connectivity_multiplier: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub host_bonus: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub desired_hostable_peers_multiplier: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub maximum_hostable_peers_multiplier: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub unbounded_hostable_peers_multiplier: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub latency_table_multiplier: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub gamestate_bonus: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub open_nat_bonus: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub hard_drive_bonus: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub local_user_table_multiplier: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub host_rating_multiplier: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub connectivity_rating_multiplier: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub synchronous_hostable_peers_multiplier: u32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_bandwidth_configuration {
    pub bandwidth_outlier_discard_fraction: f32,
    pub bandwidth_minimum_measurement_count: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub bandwidth_safety_margin_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub bandwidth_minimum_known_good_bps: u32,
    pub bandwidth_tracking_minimum_duration_msec: i32,
    pub bandwidth_tracking_maximum_satiation: f32,
    pub bandwidth_dispute_minimum_count: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub bandwidth_dispute_threshold_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub bandwidth_dispute_increase_bps: u32,
    pub host_speculative_migration_check_interval_msec: i32,
    pub host_speculative_migration_check_interval_custom_msec: i32,
    pub host_speculative_migration_check_interval_matchmaking_msec: i32,
    pub host_speculative_migration_remigrate_interval_msec: i32,
    pub host_speculative_migration_required_lobby_peer_connectivity_difference: i32,
    pub host_speculative_migration_required_match_host_rating_difference: i32,
    pub host_speculative_migration_required_match_host_bandwidth_difference: i32,
    pub host_speculative_migration_required_custom_host_rating_difference: i32,
    pub host_speculative_migration_required_custom_host_bandwidth_difference: i32,
    pub host_preference_latency_table: StaticArray<u8, 20>,
    pub host_preference_local_user_table: StaticArray<u8, 5>,
    #[brw(align_before = 4)]
    pub host_preferences: StaticArray<s_bandwidth_configuration_host_preference_table, 4>,
    pub host_preference_connectivity_rankings_spread: i32,
    pub host_preference_host_rankings_spread: i32,
    pub upstream_bandwidth_absolute_minimum_bps: StaticArray<i32, 17>,
    pub upstream_bandwidth_requirement_bps: StaticArray<i32, 17>,
    pub upstream_bandwidth_desired_bps: StaticArray<i32, 17>,
    pub minimum_player_restricted_count: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub minimum_host_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub minimum_host_downstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub minimum_host_delegation_advantage_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub good_host_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub good_host_downstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub minimum_voice_repeater_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub minimum_voice_repeater_downstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub voice_channel_bandwidth_bps: u32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_life_cycle_handler_joining_configuration {
    pub network_session_migration_wait_timeout_msec: i32,
    pub network_session_migration_disband_timeout_msec: i32,
    pub join_remote_squad_player_reservation_timeout_msec: i32,
    pub request_migration_start_rety_interval_msec: i32,
    pub request_migration_abort_rety_interval_msec: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub joining_search_qos_bps: u32,
    pub join_timeout_msec: i32,
    pub desperation_wait_time_seconds: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_life_cycle_handler_matchmaking_configuration {
    pub perform_nat_check: s_bool,
    #[brw(align_before = 4)]
    pub matchmaking_strict_nat_host_percentage: f32,
    pub matchmaking_start_failure_wait_time_ms: i32,
    pub matchmaking_find_match_joinable_session_threshold: i32,
    pub matchmaking_find_match_join_wait_time_ms: i32,
    pub matchmaking_find_match_search_results_stale_ms: i32,
    pub matchmaking_gather_join_wait_time_ms: i32,
    pub matchmaking_search_give_up_time_seconds: i32,
    pub join_request_wait_time_ms: i32,
    pub prepare_map_display_map_during_loading: s_bool,
    #[brw(align_before = 4)]
    pub prepare_map_veto_timer_seconds: i32,
    pub prepare_map_minimum_load_time_seconds: i32,
    pub prepare_map_countdown_timer_seconds: i32,
    pub prepare_map_vetoed_countdown_timer_seconds: i32,
    pub prepare_map_veto_failed_countdown_timer_seconds: i32,
    pub end_match_write_stats_boot_threshold_seconds: i32,
    pub arbitration_wait_for_establishment_and_connectivity_threshold_seconds: i32,
    pub arbitration_wait_for_completion_threshold_seconds: i32,
    pub post_match_return_to_pre_game_lobby: s_bool,
    #[brw(align_before = 4)]
    pub post_match_stats_refresh_time: i32,
    pub warning_toast_minimum_time_seconds: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_life_cycle_handler_in_game_configuration {
    pub simulation_aborted_host_delay_ms: i32,
    pub simulation_aborted_peer_delay_ms: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_life_cycle_configuration {
    pub joining: s_life_cycle_handler_joining_configuration,
    pub matchmaking: s_life_cycle_handler_matchmaking_configuration,
    pub in_game: s_life_cycle_handler_in_game_configuration,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_logic_session_tracker_configuration {
    pub unsuitable_session_cache_count: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub qos_bps: u32,
    pub default_qos_refresh_interval_msec: i32,
    pub full_qos_timeout_msec: i32,
    pub maximum_target_sessions_per_default_qos_task: i32,
    pub maximum_qos_tasks: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_logic_matchmaking_desirability {
    pub add_to_match_desirability_bonus: i32,
    pub combined_player_count_per_player_desirability_bonus: i32,
    pub fill_out_match_desirability_bonus: i32,
    pub needed_party_size_desirability_bonus: i32,
    pub gather_time_bonus_threshold_seconds: i32,
    pub gather_time_desirability_bonus: i32,
    pub search_time_bonus_threshold_seconds: i32,
    pub search_time_desirability_bonus: i32,
    pub average_skill_range_for_desirability_bonus: i32,
    pub average_skill_desirability_bonus_factor: i32,
    pub average_mu_range_for_desirability_bonus: f32,
    pub average_mu_desirability_bonus_factor: i32,
    pub average_experience_rank_range_for_desirability_bonus: i32,
    pub average_experience_rank_desirability_bonus_factor: i32,
    pub good_host_desirability_bonus: i32,
    pub language_match_desirability_bonus: i32,
    pub gamer_region_match_desirability_bonus: i32,
    pub gamer_zone_match_desirability_bonus: i32,
    pub quitter_match_desirability_bonus: i32,
    pub dlc_match_desirability_bonus: i32,
    pub maximum_ping_for_desirability_bonus: i32,
    pub ping_desirability_bonus_interval: i32,
    pub desirability_bonus_per_ping_interval: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_logic_matchmaking_seeker_configuration {
    pub session_search_wait_time_ms: i32,
    pub failed_session_search_wait_time_ms: i32,
    pub final_session_search_query_minimum_time_ms: i32,
    pub search_stage_strict_skill_round_limit: i32,
    pub search_stage_strict_skill_initial_mu_range: f32,
    pub search_stage_strict_skill_final_mu_range: f32,
    pub search_stage_strict_skill_initial_desired_skill_range: i32,
    pub search_stage_strict_skill_final_desired_skill_range: i32,
    pub search_stage_strict_skill_initial_average_skill_range: i32,
    pub search_stage_strict_skill_final_average_skill_range: i32,
    pub search_stage_strict_skill_initial_average_experience_rank_range: i32,
    pub search_stage_strict_skill_connection_threshold_ms: i32,
    pub search_stage_skill_round_limit: i32,
    pub search_stage_skill_connection_threshold_ms: i32,
    pub search_stage_any_round_limit: i32,
    pub search_stage_any_final_connection_threshold_ms: i32,
    pub search_stage_any_final_mu_range: f32,
    pub search_stage_any_final_average_skill_range: i32,
    pub search_stage_any_final_desired_skill_range: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_logic_leaderboard_configuration {
    pub consecutive_failed_download_threshold: i32,
    pub milliseconds_between_failed_downloads: i32,
    pub milliseconds_between_consecutive_failed_downloads: i32,
    pub refresh_request_wait_time_ms: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_session_interface_configuration {
    pub peer_properties_interval_msec: i32,
    pub user_addition_interval_msec: i32,
    pub user_removal_interval_msec: i32,
    pub user_properties_interval_msec: i32,
    pub local_specific_parameter_propagation_msec: i32,
    pub ready_for_next_match_wait_time_milliseconds: i32,
    pub stat_replication_wait_threshold_milliseconds: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_qos_reply_block_configuration {
    pub qos_listener_update_interval: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub disabled_qos_reply_bandwidth_bps: u32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_session_qos_reply_block_configuration {
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub out_of_game_qos_reply_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub in_game_qos_reply_bandwidth_bps: u32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_logic_configuration {
    pub session_tracker: s_logic_session_tracker_configuration,
    pub desirability: s_logic_matchmaking_desirability,
    pub seeker: s_logic_matchmaking_seeker_configuration,
    pub leaderboard: s_logic_leaderboard_configuration,
    pub session_interface: s_session_interface_configuration,
    pub base_qos_reply_block: s_qos_reply_block_configuration,
    pub squad_qos_reply_block: s_session_qos_reply_block_configuration,
    pub group_qos_reply_block: s_session_qos_reply_block_configuration,
    pub maximum_players_in_coop: i32,
    pub maximum_players_in_forge: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_banhammer_configuration {
    pub machine_file_refresh_seconds: i32,
    pub machine_file_refresh_threshold_seconds: i32,
    pub user_file_refresh_seconds: i32,
    pub user_file_refresh_threshold_seconds: i32,
    pub host_chance_reduction_percentage: i32,
    pub idle_controller_timeout_seconds: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_zoom_relevance {
    pub zoom_0_tolerance: f32,
    pub zoom_1_tolerance: f32,
    pub zoom_0_relevance_bonus: f32,
    pub zoom_1_relevance_bonus: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_control_relevance {
    pub zero_relevance_distance: f32,
    pub max_relevance: f32,
    pub min_relevance: f32,
    pub min_period: i32,
    pub max_period: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_position_relevance {
    pub distance_to_player_threshold: f32,
    pub aiming_vector_high_tolerance: f32,
    pub aiming_vector_medium_tolerance: f32,
    pub distance_to_player_medium_tolerance: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_netdebug_configuration {
    pub bar_maximum_count: i32,
    pub axis_bounds: StaticArray<StaticArray<i32, 2>, 4>
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_view_configuration {
    pub game_results_update_interval_msec: i32,
    pub synchronous_client_block_duration_msec: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_shared_configuration {
    pub action_persist_time: f32,
    pub simulation_event_projectile_supercombine_request_fraction: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_world_configuration {
    pub maximum_catchup_views: i32,
    pub join_timeout: i32,
    pub host_join_minimum_wait_time: i32,
    pub host_join_timeout: i32,
    pub join_total_wait_timeout: i32,
    pub pause_game_required_machines_fraction: f32,
    pub join_activation_blocking_machines_fraction: f32,
    pub maximum_catchup_attempts: i32,
    pub catchup_failure_timeout: i32,
    pub client_join_failure_count: i32,
    pub client_activation_failure_timeout: i32,
    pub game_simulation_queue_danger_allocation_size_percentage: f32,
    pub game_simulation_queue_danger_allocation_count_percentage: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_event_configuration {
    pub constant_priority: f32,
    pub cancel_timer_milliseconds: i32,
    pub zero_relevance_distance: f32,
    pub minimum_priority: f32,
    pub maximum_priority: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_entity_creation_configuration {
    pub constant_priority: f32,
    pub creation_zero_relevance_distance: f32,
    pub creation_minimum_priority: f32,
    pub creation_maximum_priority: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_entity_update_configuration {
    pub constant_priority: f32,
    pub zero_relevance_distance: f32,
    pub minimum_relevance: f32,
    pub maximum_relevance: f32,
    pub minimum_period: i32,
    pub maximum_period: i32,
    pub normal_minimum_priority: f32,
    pub normal_maximum_priority: f32,
    pub delayed_time_threshold: i32,
    pub delayed_minimum_priority: f32,
    pub delayed_maximum_priority: f32,
    pub maximum_priority: f32,
    pub player_priority: f32,
    pub dead_priority: f32,
    pub in_motion_by_unit: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_entity_configuration {
    pub creation_configuration: s_simulation_entity_creation_configuration,
    pub update_configuration: s_simulation_entity_update_configuration,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_warping_configuration {
    pub simulation_position_update_object_corrective_accept_tolerance: f32,
    pub simulation_position_update_object_predicted_accept_tolerance: f32,
    pub simulation_position_update_vehicle_corrective_accept_tolerance: f32,
    pub simulation_position_update_vehicle_predicted_accept_tolerance: f32,
    pub position_update_recent_seconds: f32,
    pub position_update_minimum_distance: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_weapon_configuration {
    pub trigger_recent_spew_time: f32,
    pub prediction_delay_timer: f32,
    pub predicted_fire_allow_ratio: f32,
    pub predicted_fire_always_allow_threshold: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_configuration {
    pub zoom_relevance: s_simulation_zoom_relevance,
    pub control_relevance: s_simulation_control_relevance,
    pub position_relevance: s_simulation_position_relevance,
    pub netdebug: s_simulation_netdebug_configuration,
    pub view: s_simulation_view_configuration,
    pub shared: s_simulation_shared_configuration,
    pub world: s_simulation_world_configuration,
    pub events: StaticArray<s_simulation_event_configuration, 34>,
    pub entities: StaticArray<s_simulation_entity_configuration, 22>,
    pub warping: s_simulation_warping_configuration,
    pub weapon: s_simulation_weapon_configuration,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_event_manager_view_configuration {
    pub replication_event_maximum_blocked_time: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_replication_control_view {
    pub base_non_player_motion_priority: f32,
    pub max_priority_threshold: i32,
    pub max_priority: f32,
    pub medium_priority_base: f32,
    pub medium_priority_relevance_scale: f32,
    pub min_priority_base: f32,
    pub min_priority_relevance_scale: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_replication_configuration {
    pub event_manager_view: s_event_manager_view_configuration,
    pub replication_control_view: s_replication_control_view,
    pub maximum_requests_to_send_in_one_frame: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_session_configuration {
    pub session_recreate_timeout_msec: i32,
    pub join_request_interval_msec: i32,
    pub join_secure_connection_timeout_msec: i32,
    pub join_initial_update_timeout_msec: i32,
    pub join_time_to_hold_in_join_queue_msec: i32,
    pub join_notify_client_join_in_queue_interval_msec: i32,
    pub join_abort_interval_msec: i32,
    pub join_abort_timeout_msec: i32,
    pub host_rejoin_accept_timeout_msec: i32,
    pub leave_timeout_msec: i32,
    pub leave_request_interval_msec: i32,
    pub host_handoff_initiate_timeout_msec: i32,
    pub host_handoff_selection_delay_msec: i32,
    pub host_handoff_selection_timeout_msec: i32,
    pub host_transition_timeout_msec: i32,
    pub host_reestablish_timeout_msec: i32,
    pub host_reestablish_maximum_send_to_original_host_delay_msec: i32,
    pub election_failsafe_timeout_msec: i32,
    pub election_peer_timeout_msec: i32,
    pub election_ignore_dissension_msec: i32,
    pub guaranteed_election_send_interval_msec: i32,
    pub time_synchronization_interval_msec: i32,
    pub time_synchronization_retry_msec: i32,
    pub minimum_election_send_interval_msec: i32,
    #[brw(pad_before = 3)]
    pub allow_third_party_host_elections: s_bool,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_observer_configuration {
    pub secure_connect_attempts: i32,
    pub secure_connect_intervals: StaticArray<i32, 8>,
    pub connect_attempts: i32,
    pub connect_intervals: StaticArray<i32, 8>,
    pub non_simulation_reconnect_attempts: i32,
    pub non_simulation_reconnect_intervals: StaticArray<i32, 8>,
    pub reconnect_attempts: i32,
    pub reconnect_intervals: StaticArray<i32, 8>,
    pub death_recovery_time: i32,
    pub heartbeat_send_timeout: i32,
    pub connection_active_send_timeout: i32,
    pub connection_alive_send_timeout: i32,
    pub connection_alive_receive_timeout: i32,
    pub connection_drop_minimum_active_time: i32,
    pub connection_drop_receive_timeout: i32,
    pub synchronous_connection_drop_minimum_active_time: i32,
    pub synchronous_connection_drop_receive_timeout: i32,
    pub minimum_undesired_connection_timeout_msec: i32,
    pub maximum_undesired_connection_timeout_msec: i32,
    pub safety_window_threshold: f32,
    pub safety_packet_interval: f32,
    pub safety_packet_maximum_interval: f32,
    pub packet_rate_multiplier_count: i32,
    pub packet_rate_multipliers: StaticArray<f32, 16>,
    pub packet_window_minimum_bytes: i32,
    pub voice_fraction_authority: f32,
    pub voice_fraction_client: f32,
    pub voice_fraction_non_simulation: f32,
    pub voice_maximum_packet_rate: f32,
    pub voice_receive_fast_acknowledge_time: i32,
    pub bandwidth_interval: i32,
    pub loss_detection_window_size: i32,
    pub flood_max_aperture_msec: i32,
    pub flood_packet_fraction: f32,
    pub desired_payload_bytes_out_of_game: i32,
    pub desired_payload_bytes_simulation: i32,
    pub desired_minimum_packet_rate_multiplier: f32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub minimum_bandwidth_bps: u32,
    pub bandwidth_adjust_interval: i32,
    pub bandwidth_flood_channel_fraction: f32,
    pub bandwidth_flood_check_threshold: i32,
    pub bandwidth_flood_backoff_kbps: i32,
    pub bandwidth_flood_backoff_repeat_interval: i32,
    pub bandwidth_clear_check_threshold: i32,
    pub bandwidth_check_threshold_maximum: i32,
    pub bandwidth_initial_backoff_kbps: i32,
    pub bandwidth_known_good_minimum_kbps: i32,
    pub bandwidth_increment_known_bad_threshold_kbps: i32,
    pub bandwidth_increment_kbps: i32,
    #[brw(align_after = 4)]
    pub generate_stream_events: s_bool,
    pub observer_stream_expansion_interval_msec: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_minimum_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_maximum_bps: u32,
    pub stream_initial_startup_msec: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_initial_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_initial_total_bps_unreliable: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_initial_total_bps_reliable: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_initial_minimum_bps_per_stream: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_minimum_great_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_out_of_game_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_distributed_client_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_distributed_host_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_synchronous_client_initial_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_synchronous_host_initial_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_synchronous_joining_host_initial_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_synchronous_client_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_synchronous_host_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_synchronous_joining_host_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_synchronous_non_joining_host_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_synchronous_client_minimum_upstream_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_synchronous_host_minimum_upstream_bandwidth_bps: u32,
    pub stream_rtt_noise_msec: i32,
    pub stream_minimum_rtt_msec: i32,
    pub stream_rtt_average_gain_bits: i32,
    pub stream_loss_window_size: i32,
    pub stream_loss_throttle_fraction: f32,
    pub stream_throttle_rtt_multiplier: i32,
    pub stream_throttle_reduce_multiplier: f32,
    pub stream_throttle_cookie_event_count: i32,
    pub stream_throttle_cookie_backoff_threshold: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_bandwidth_step_bps: u32,
    pub stream_bandwidth_step_max_fraction: f32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_bandwidth_backoff_bps: u32,
    pub stream_bandwidth_backoff_max_fraction: f32,
    pub stream_period_maximum_msec: i32,
    pub stream_growth_period_minimum_msec: i32,
    pub stream_throughput_recording_period_minimum_msec: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_throughput_satiated_bandwidth_bps: u32,
    pub stream_throughput_satiated_stream_fraction: f32,
    pub stream_satiation_timeout_msec: i32,
    pub stream_congestion_bandwidth_average_gain_bits: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stream_congestion_offender_bandwidth_increment_bps: u32,
    pub stream_congestion_offender_timeout_msec: i32,
    pub stream_growth_maximum_count: i32,
    pub stream_growth_maximum_fraction: f32,
    pub stream_growth_desire_maximum: f32,
    pub stream_growth_desire_delay_multiplier: i32,
    pub stream_growth_desire_penalty_bandwidth_multiplier: f32,
    pub stream_growth_out_of_game_maximum_rtt_increase_msec: i32,
    pub stream_growth_simulation_maximum_rtt_increase_msec: i32,
    pub stream_non_growth_simulation_maximum_rtt_increase_msec: i32,
    pub stream_growth_backoff_rtt_event_threshold: i32,
    pub stream_growth_incremental_rtt_event_threshold: i32,
    pub stream_growth_incremental_rtt_increase_msec: i32,
    pub stream_cautious_expansion_limited_periods_count: i32,
    pub stream_cautious_expansion_instability_timeout_msec: i32,
    pub bandwidth_monitor_period_count: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub bandwidth_monitor_limitation_bps: u32,
    pub bandwidth_monitor_constriction_fraction: f32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub bandwidth_monitor_constriction_threshold_bps: u32,
    pub client_badness_rating_threshold: i32,
    pub bad_bandwidth_throughput_threshold: i32,
    pub disable_bad_client_anticheating: s_bool,
    pub disable_bad_connectivity_anticheating: s_bool,
    pub disable_bad_bandwidth_anticheating: s_bool,
    #[brw(align_before = 4)]
    pub initial_timeout: i32,
    pub mini_period_minimum_duration: i32,
    pub mini_period_minimum_rtt_deviation: i32,
    pub stream_wants_more_bandwidth_fudge_factor: i32,
    pub stream_wants_more_bandwidth_fudge_factor_small: i32,
    pub stream_wants_more_allocation_fudge_factor: i32,
    pub stream_wants_more_allocation_fudge_factor_small: i32,
    pub stream_maximum_instability_value: i32,
    pub stream_probe_failure_limit: i32,
    pub stream_rebalance_interval_msec: i32,
    pub packet_loss_rate_gain_bits: i32,
    pub packet_loss_rate_deviation_gain_bits: i32,
    pub maximum_rtt_increase_msec: i32,
    pub maximum_lost_packet_rate_increase: i32,
    pub minimum_packet_loss_deviation: i32,
    pub minimum_probe_packet_loss_deviation: i32,
    pub stream_maximum_bandwidth_maximum_delta: i32,
    pub stream_maximum_bandwidth_skip_max: i32,
    pub packet_loss_deviation_adjustment: i32,
    pub packet_rate_deviation_adjustment: i32,
    pub consecutive_rate_failures_before_badness: i32,
    pub consecutive_latency_failures_before_badness: i32,
    pub consecutive_packet_loss_failures_before_badness: i32,
    pub consecutive_packet_loss_spike_failures_before_badness: i32,
    pub badness_minimum_host_to_client_bandwidth: i32,
    pub badness_minimum_allocated_host_to_client_bandwidth: i32,
    pub badness_minimum_client_to_host_bandwidth: i32,
    pub badness_minimum_allocated_client_to_host_bandwidth: i32,
    pub badness_maximum_latency_msec: i32,
    pub badness_maximum_packet_loss_rate: i32,
    pub badness_maximum_packet_loss_spike_count: i32,
    pub stream_congestion_rtt_multiplier: i32,
    pub remote_client_downstream_usage_multiplier: i32,
    pub remote_host_downstream_usage_multiplier: i32,
    pub throttle_congested_stream_bandwidth_multiplier: i32,
    pub throttle_noncongested_stream_bandwidth_multiplier: i32,
    pub bandwidth_estimate_multiplier_reliable: i32,
    pub bandwidth_estimate_multiplier_unreliable: i32,
    pub bandwidth_estimate_multiplier_qos: i32,
    pub qos_to_live_interval_msec: i32,
    pub synchronous_joining_maximum_stream_growth_shift: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub maximum_aggressive_total_growth_bandwidth_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub maximum_cautious_total_growth_bandwidth_bps: u32,
    pub client_probe_base_delay_msec: i32,
    pub client_probe_additional_client_delay_msec: i32,
    pub client_probe_variability: i32,
    pub stream_stability_deviation_multiplier: i32,
    pub stream_growth_base_upstream_shift: i32,
    pub stream_growth_base_upstream_shift_max: i32,
    pub stream_fail_all_failed_related_probes: i32,
    pub maximum_consecutive_probe_successes: i32,
    pub minimum_packet_rate_for_automatic_congestion: i32,
    pub maximum_rtt_for_automatic_congestion: i32,
    pub do_collateral_last_resort_throttle: s_bool,
    pub release_throttle_lock_on_first_congest: s_bool,
    pub round_robin_probes: s_bool,
    pub fail_all_member_probes_together: s_bool,
    pub mark_throttled_stream_with_one_failure: s_bool,
    pub check_single_stream_overprobe: s_bool,
    pub use_deviation_only_for_related_rtt_timeout: s_bool,
    pub fast_probe_failed_streams: s_bool,
    pub packet_loss_spike_threshold: i32,
    pub packet_loss_spike_minimum_packet_count: i32,
    pub packet_loss_spike_skip_averaging_any_spike: i32,
    pub stable_probe_start_stream_growth_base: i32,
    pub stable_probe_start_stream_growth_shift_shift: i32,
    pub stable_probe_maximum_stream_growth_shift: i32,
    pub stable_probe_query_time_msec: i32,
    pub stable_probe_query_time_minimum_msec: i32,
    pub stable_probe_settle_time_msec: i32,
    pub stable_probe_settle_bandwidth_multiplier: f32,
    pub stable_probe_throttle_minimum_rollback: i32,
    pub stable_probe_recover_minimum_time_msec: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stable_probe_overprobe_minimum_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub stable_probe_overprobe_maximum_bps: u32,
    pub stable_probe_overprobe_transmit_ratio: i32,
    pub initial_probe_start_stream_growth_base: i32,
    pub initial_probe_start_stream_growth_shift_shift: i32,
    pub initial_probe_maximum_stream_growth_shift: i32,
    pub initial_probe_query_time_msec: i32,
    pub initial_probe_query_time_minimum_msec: i32,
    pub initial_probe_settle_time_msec: i32,
    pub initial_probe_settle_bandwidth_multiplier: f32,
    pub initial_probe_throttle_minimum_rollback: i32,
    pub initial_probe_recover_minimum_time_msec: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub initial_probe_overprobe_minimum_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub initial_probe_overprobe_maximum_bps: u32,
    pub initial_probe_overprobe_transmit_ratio: i32,
    pub fast_probe_start_stream_growth_base: i32,
    pub fast_probe_start_stream_growth_shift_shift: i32,
    pub fast_probe_maximum_stream_growth_shift: i32,
    pub fast_probe_query_time_msec: i32,
    pub fast_probe_query_time_minimum_msec: i32,
    pub fast_probe_settle_time_msec: i32,
    pub fast_probe_settle_bandwidth_multiplier: f32,
    pub fast_probe_throttle_minimum_rollback: i32,
    pub fast_probe_recover_minimum_time_msec: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub fast_probe_overprobe_minimum_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub fast_probe_overprobe_maximum_bps: u32,
    pub fast_probe_overprobe_transmit_ratio: i32,
    pub slow_probe_start_stream_growth_base: i32,
    pub slow_probe_start_stream_growth_shift_shift: i32,
    pub slow_probe_maximum_stream_growth_shift: i32,
    pub slow_probe_query_time_msec: i32,
    pub slow_probe_query_time_minimum_msec: i32,
    pub slow_probe_settle_time_msec: i32,
    pub slow_probe_settle_bandwidth_multiplier: f32,
    pub slow_probe_throttle_minimum_rollback: i32,
    pub slow_probe_recover_minimum_time_msec: i32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub slow_probe_overprobe_minimum_bps: u32,
    #[serde(with = "SerHex::<StrictCapPfx>")]
    pub slow_probe_overprobe_maximum_bps: u32,
    pub slow_probe_overprobe_transmit_ratio: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_channel_configuration {
    pub connect_request_interval_msec: i32,
    pub connect_request_count: i32,
    pub connect_request_timeout_msec: i32,
    pub establish_timeout_msec: i32,
    pub packet_statistics_interval: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_connection_configuration {
    pub sequence_advance_rate: i32,
    pub discard_ancient_reply_sequence_threshold: i32,
    pub packet_skipped_outoforder_threshold: i32,
    pub retain_lost_packets_msec: i32,
    pub latency_average_gain_bits: i32,
    pub latency_deviation_gain_bits: i32,
    pub timeout_deviations: i32,
    pub timeout_minimum_tolerance_msec: i32,
    pub timeout_minimum_msec: i32,
    pub inactive_timeout_deviations: i32,
    pub inactive_timeout_minimum_tolerance_msec: i32,
    pub inactive_timeout_minimum_msec: i32,
    pub initial_latency_minimum_msec: i32,
    pub initial_latency_average_msec: i32,
    pub initial_latency_deviation_msec: i32,
    pub initial_timeout_msec: i32,
    pub backoff_increment_msec: i32,
    pub backoff_max_msec: i32,
    pub backoff_decrement_msec: i32,
    pub bandwidth_warning_latency_multiplier: f32,
    pub bandwidth_warning_latency_minimum_increase: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_delivery_configuration {
    pub channel: s_channel_configuration,
    pub connection: s_connection_configuration,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_transport_configuration {
    pub address_resolution_timeout_msec: i32,
    pub qos_probe_count: StaticArray<i32, 2>,
    pub qos_probe_bps: StaticArray<i32, 2>,
    pub qos_upstream_cap_enabled: s_bool,
    #[brw(align_before = 4)]
    pub qos_upstream_cap_upstream_modifier_percentage: i32,
    pub qos_upstream_cap_correction_modifier: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_voice_configuration {
    pub push_to_talk_inactivity_threshold_seconds: f32,
    pub maximum_push_to_talk_time_seconds: f32,
    pub clients_can_be_preferred_consumers_of_voice_repeater: s_bool,
    #[brw(align_before = 4)]
    pub open_channel_player_count: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_data_mine_configuration {
    pub ui_upload_record_threshold: i32,
    pub ui_upload_time_threshold: i32,
    #[brw(align_after = 4)]
    pub record_uploads_prevent_game_from_starting: s_bool,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_griefer_configuration {
    pub betrayal_decrement_time: u32,
    pub eject_decrement_time: u32,
    pub betrayal_cutoff: f32,
    pub ejection_cutoff: i32,
    pub UNUSED_friendly_assist_amount: f32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_network_memory_configuration {
    pub network_heap_large_size: i32,
    pub network_heap_small_size: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_user_interface {
    pub allow_matchmaking_abort_interval_msec: i32,
    pub networked_playback_maximum_player_count: i32,
    pub basic_training_completion_minimum_games_completed: i32,
    pub basic_training_completion_minimum_experience: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_skill_level_configuration {
    pub bonus_skill_level_wins_required: StaticArray<i32, 50>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_experience_configuration {
    pub experience_for_rank: StaticArray<i32, 56>,
    pub skill_for_rank: StaticArray<i32, 14>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_alpha_configuration {
    pub disable_game: s_bool,
    pub disable_custom_games: s_bool,
    #[brw(align_before = 4)]
    pub ui_level: e_alpha_configuration_ui_level,
    pub maximum_multiplayer_split_screen: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_crash_handling_configuration {
    pub display_crash_handling_ui: s_bool,
    #[brw(align_before = 4)]
    pub minidump_generation: e_minidump_generation,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_lsp_configuration {
    pub port_range_start: i32,
    pub port_range_count: i32,
    pub server_retry_count: i32,
    pub search_results_fresh_milliseconds: i32,
    pub recent_activity_milliseconds: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_map_information {
    pub map_id: u32,
    pub map_status: e_map_status,
    pub dlc_path_index: i32,
    pub dlc_pack: e_dlc_pack,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_map_configuration {
    pub map_list: StaticArray<s_map_information, 32>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct s_dlc_path {
    pub path: StaticString<0x80>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_dlc_paths {
    pub paths: StaticArray<s_dlc_path, 16>,
}

impl BinRead for s_dlc_path {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: binrw::Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        // Read the `StaticString` field, ignoring the first bit
        let mut buffer = vec![0u8; 0x80];
        reader.read_exact(&mut buffer)?;

        // Ignore the first bit of the first byte
        buffer[0] &= 0b0111_1111;
        let mut cursor = Cursor::new(buffer);

        // Convert the buffer to a StaticString
        let path: StaticString<0x80> = cursor.read_type(endian)?;

        Ok(s_dlc_path { path })
    }
}

impl BinWrite for s_dlc_path {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: binrw::Endian, _args: Self::Args<'_>) -> BinResult<()> {
        // Convert the StaticString to bytes
        let string: String = self.path.get_string();
        let mut buffer = string.into_bytes();
        buffer.resize(0x80, 0);

        // Ensure the first bit is set in the first byte
        buffer[0] |= 0b1000_0000;

        // Write the buffer to the output
        writer.write_all(buffer.as_slice())?;
        Ok(())
    }
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_chicken_switches {
    pub allow_no_hdd_network_coop: s_bool,
    pub allow_matched_hdd_network_coop: s_bool,
    pub disallow_cross_language_coop: s_bool,
    pub disable_prefer_good_connection_changes: s_bool,
    pub allow_vidmaster_alpha_achievement: s_bool,
    #[brw(align_after = 4)]
    pub au2_lsp_acquire_fix_only_enable_if_wb2_is_present: s_bool,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_determinism_configuration {
    pub determinism_version: i32,
    pub determinism_compatible_version: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_network_configuration {
    pub config_download: s_network_file_download_configuration,
    pub bandwidth: s_bandwidth_configuration,
    pub life_cycle: s_life_cycle_configuration,
    pub logic: s_logic_configuration,
    pub banhammer: s_banhammer_configuration,
    pub simulation: s_simulation_configuration,
    pub replication: s_replication_configuration,
    pub session: s_session_configuration,
    pub observer: s_observer_configuration,
    pub delivery: s_delivery_configuration,
    pub transport: s_transport_configuration,
    pub voice: s_voice_configuration,
    pub data_mine: s_data_mine_configuration,
    pub griefer_config: s_griefer_configuration,
    pub memory: s_network_memory_configuration,
    pub user_interface: s_user_interface,
    pub skill_level_configuration: s_skill_level_configuration,
    pub experience_configuration: s_experience_configuration,
    pub hopper_experience_configuration: s_experience_configuration,
    pub alpha_configuration: s_alpha_configuration,
    pub crash_handling_configuration: s_crash_handling_configuration,
    pub lsp_configuration: s_lsp_configuration,
    pub map_configuration: s_map_configuration,
    pub dlc_paths: s_dlc_paths,
    pub chicken_switches: s_chicken_switches,
    pub determinism_configuration: s_determinism_configuration,
}


