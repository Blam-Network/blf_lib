use libc::{time64_t};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blf_chunk;
use blf_lib::io::bitstream::{create_bitstream_reader, create_bitstream_writer, e_bitstream_byte_order};
use blf_lib::types::array::StaticArray;
use blf_lib::types::byte_limited_utf8_string::StaticString;
use blf_lib_derivable::blf::chunks::SerializableBlfChunk;
use crate::io::bitstream::close_bitstream_writer;

#[derive(Clone, Default, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub struct s_game_hopper_custom_category {
    pub category_identifier: u16,
    pub category_image_index: u8,
    pub category_name: StaticString<32>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct c_hopper_configuration {
    pub hopper_name: StaticString<32>,
    pub game_set_hash: s_network_http_request_hash,
    pub hopper_identifier: u16,
    pub hopper_category: u16,
    pub hopper_type: u8,
    pub image_index: u8,
    pub xlast_index: u8,
    pub rich_presence_id: u16,
    // __data49[0x7];
    pub start_time: time64_t,
    pub end_time: time64_t,
    pub hopper_regions: u32,
    pub minimum_base_xp: u32,
    pub maximum_base_xp: u32,
    pub minimum_games_played: u32,
    pub maximum_games_played: u32,
    pub minimum_party_size: u32,
    pub maximum_party_size: u32,
    pub hopper_access_bit: i8,
    pub account_type_access: u8,
    pub require_all_party_members_meet_games_played_requirements: bool,
    pub require_all_party_members_meet_experience_requirements: bool,
    pub require_all_party_members_meet_access_requirements: bool,
    pub require_all_party_members_meet_live_account_access_requirements: bool,
    pub hide_hopper_from_games_played_restricted_players: bool,
    pub hide_hopper_from_experience_restricted_players: bool,
    pub hide_hopper_from_access_restricted_players: bool,
    pub hide_hopper_from_live_account_access_restricted_players: bool,
    pub hide_hopper_due_to_time_restriction: bool,
    pub requires_all_downloadable_maps: bool,
    pub veto_enabled: bool,
    pub guests_allowed: bool,
    pub require_hosts_on_multiple_teams: bool,
    pub stats_write: u8,
    pub language_filter: u8,
    pub country_code_filter: u8,
    pub gamerzone_filter: u8,
    pub quitter_filter_percentage: u8,
    pub quitter_filter_maximum_party_size: u8,
    pub rematch_countdown_timer: u16,
    pub rematch_group_formation: u8,
    pub repeated_opponents_to_consider_for_penalty: u8,
    pub repeated_opponents_experience_threshold: u8,
    pub repeated_opponents_skill_throttle_start: u8,
    pub repeated_opponents_skill_throttle_stop: u8,
    pub maximum_total_matchmaking_seconds: u16,
    pub gather_start_game_early_seconds: u16,
    pub gather_give_up_seconds: u16,
    pub chance_of_gathering: [u8;16],
    pub experience_points_per_win: u8,
    pub experience_penalty_per_drop: u8,
    pub minimum_mu_per_level: StaticArray<f32, 49>,
    pub maximum_skill_level_match_delta: StaticArray<u8, 50>,
    pub trueskill_sigma_multiplier: f32,
    pub trueskill_beta_performance_variation: f32,
    pub trueskill_tau_dynamics_factor: f32,
    pub trueskill_adjust_tau_with_update_weight: bool,
    pub trueskill_draw_probability: u8,
    pub trueskill_hillclimb_w0: u8,
    pub trueskill_hillclimb_w50: u8,
    pub trueskill_hillclimb_w100: u8,
    pub trueskill_hillclimb_w150: u8,
    pub skill_update_weight_s0: u8,
    pub skill_update_weight_s10: u8,
    pub skill_update_weight_s20: u8,
    pub skill_update_weight_s30: u8,
    pub skill_update_weight_s40: u8,
    pub skill_update_weight_s50: u8,
    pub quality_update_weight_q0: u8,
    pub quality_update_weight_q25: u8,
    pub quality_update_weight_q50: u8,
    pub quality_update_weight_q75: u8,
    pub quality_update_weight_q100: u8,
    pub pre_match_voice: u8,
    pub in_match_voice: u8,
    pub post_match_voice: u8,
    pub restrict_open_channel: bool,
    // ffa
    pub minimum_player_count: u8,
    pub maximum_player_count: u8,
    // unranked teams
    pub team_count: u8,
    pub minimum_team_size: u8,
    pub maximum_team_size: u8,
    pub allow_uneven_teams: bool,
    pub allow_parties_to_split: bool,
    // ranked teams
    pub maximum_team_imbalance: u8,
    pub big_squad_size_threshold: u8,
    pub maximum_big_squad_imbalance: u8,
    pub enable_big_squad_mixed_skill_restrictions: bool
}

pub const k_hopper_maximum_hopper_count: usize = 32;

blf_chunk!(
    #[Signature("mhcf")]
    #[Version(11.1)]
    pub struct s_blf_chunk_hopper_configuration_table
    {
        pub hopper_category_count: u8,
        pub hopper_categories: StaticArray<s_game_hopper_custom_category, 4>,

        pub hopper_configuration_count: u8,
        pub hopper_configurations: StaticArray<c_hopper_configuration, k_hopper_maximum_hopper_count>,
    }
);

impl s_blf_chunk_hopper_configuration_table {
    pub fn get_hopper_categories(&self) -> Vec<s_game_hopper_custom_category> {
        self.hopper_categories.get().as_slice()[0..self.hopper_category_count as usize].to_vec()
    }

    pub fn get_hopper_configurations(&self) -> Vec<c_hopper_configuration> {
        self.hopper_configurations.get().as_slice()[0..self.hopper_configuration_count as usize].to_vec()
    }
}

impl SerializableBlfChunk for s_blf_chunk_hopper_configuration_table {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
        let mut bitstream = create_bitstream_writer(0x1BC0, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        // bitstream.write_integer(self.game_entry_count as u32, 6);
        //
        // for i in 0..self.game_entry_count {
        //     let game_entry = self.game_entries[i];
        //     bitstream.write_integer(game_entry.weight, 32);
        //     bitstream.write_integer(game_entry.minimum_player_count as u32, 4);
        //     bitstream.write_bool(game_entry.skip_after_veto);
        //     bitstream.write_bool(game_entry.optional);
        //     bitstream.write_integer(game_entry.map_id, 32);
        //     bitstream.write_string_utf8(&game_entry.game_variant_file_name.get_string(), 32);
        //     bitstream.write_raw_data(&game_entry.game_variant_file_hash.data, 0x100);
        //     bitstream.write_string_utf8(&game_entry.map_variant_file_name.get_string(), 32);
        //     bitstream.write_raw_data(&game_entry.map_variant_file_hash.data, 0x100);
        // }

        close_bitstream_writer(&mut bitstream)
    }

    fn decode_body(&mut self, buffer: &[u8]) {
        let mut bitstream = create_bitstream_reader(buffer, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        self.hopper_category_count = bitstream.read_u8(3);

        for i in 0..self.hopper_category_count as usize {
            let category = &mut self.hopper_categories.get_mut()[i];
            category.category_identifier = bitstream.read_u16(16);
            category.category_image_index = bitstream.read_u8(6);
            category.category_name.set_string(&bitstream.read_string_utf8(32)).unwrap();
        }

        self.hopper_configuration_count = bitstream.read_u8(6);

        for i in 0..self.hopper_configuration_count as usize {
            let configuration = &mut self.hopper_configurations.get_mut()[i];
            configuration.hopper_name.set_string(&bitstream.read_string_utf8(32)).unwrap();
            configuration.game_set_hash = s_network_http_request_hash::try_from(bitstream.read_raw_data(0xA0)).unwrap();
            configuration.hopper_identifier = bitstream.read_u16(16);
            configuration.hopper_category = bitstream.read_u16(16);
            configuration.hopper_type = bitstream.read_u8(2);
            configuration.image_index = bitstream.read_u8(6);
            configuration.xlast_index = bitstream.read_u8(5);
            configuration.rich_presence_id = bitstream.read_u16(16);
            configuration.start_time = bitstream.read_qword(64) as time64_t;
            configuration.end_time = bitstream.read_qword(64) as time64_t;
            configuration.hopper_regions = bitstream.read_integer(32);
            configuration.minimum_base_xp = bitstream.read_integer(17);
            configuration.maximum_base_xp = bitstream.read_integer(17);
            configuration.minimum_games_played = bitstream.read_integer(17);
            configuration.maximum_games_played = bitstream.read_integer(17);
            configuration.minimum_party_size = bitstream.read_integer(4) + 1;
            configuration.maximum_party_size = bitstream.read_integer(4) + 1;
            configuration.hopper_access_bit = bitstream.read_u8(4) as i8 - 1;
            configuration.account_type_access = bitstream.read_u8(2);
            configuration.require_all_party_members_meet_games_played_requirements = bitstream.read_bool();
            configuration.require_all_party_members_meet_experience_requirements = bitstream.read_bool();
            configuration.require_all_party_members_meet_access_requirements = bitstream.read_bool();
            configuration.require_all_party_members_meet_live_account_access_requirements = bitstream.read_bool();
            configuration.hide_hopper_from_games_played_restricted_players = bitstream.read_bool();
            configuration.hide_hopper_from_experience_restricted_players = bitstream.read_bool();
            configuration.hide_hopper_from_access_restricted_players = bitstream.read_bool();
            configuration.hide_hopper_from_live_account_access_restricted_players = bitstream.read_bool();
            configuration.hide_hopper_due_to_time_restriction = bitstream.read_bool();
            configuration.pre_match_voice = bitstream.read_u8(2);
            configuration.in_match_voice = bitstream.read_u8(2);
            configuration.post_match_voice = bitstream.read_u8(2);
            configuration.restrict_open_channel = bitstream.read_bool();
            configuration.requires_all_downloadable_maps = bitstream.read_bool();
            configuration.veto_enabled = bitstream.read_bool();
            configuration.guests_allowed = bitstream.read_bool();
            configuration.require_hosts_on_multiple_teams = bitstream.read_bool();
            configuration.stats_write = bitstream.read_u8(2);
            configuration.language_filter = bitstream.read_u8(2);
            configuration.country_code_filter = bitstream.read_u8(2);
            configuration.gamerzone_filter = bitstream.read_u8(2);
            configuration.quitter_filter_percentage = bitstream.read_u8(7);
            configuration.quitter_filter_maximum_party_size = bitstream.read_u8(4);
            configuration.rematch_countdown_timer = bitstream.read_u16(10);
            configuration.rematch_group_formation = bitstream.read_u8(2);
            configuration.repeated_opponents_to_consider_for_penalty = bitstream.read_u8(7);
            configuration.repeated_opponents_experience_threshold = bitstream.read_u8(4);
            configuration.repeated_opponents_skill_throttle_start = bitstream.read_u8(4);
            configuration.repeated_opponents_skill_throttle_stop = bitstream.read_u8(4);
            configuration.maximum_total_matchmaking_seconds = bitstream.read_u16(10);
            configuration.gather_start_game_early_seconds = bitstream.read_u16(10);
            configuration.gather_give_up_seconds = bitstream.read_u16(10);

            for i in 0..configuration.chance_of_gathering.len() {
                configuration.chance_of_gathering[i] = bitstream.read_u8(7);
            }

            configuration.experience_points_per_win = bitstream.read_u8(2);
            configuration.experience_penalty_per_drop = bitstream.read_u8(2);

            for i in 0..configuration.minimum_mu_per_level.get().iter().len() {
                configuration.minimum_mu_per_level.get_mut()[i] = bitstream.read_float(32);
            }

            for i in 0..configuration.maximum_skill_level_match_delta.get().iter().len() {
                configuration.maximum_skill_level_match_delta.get_mut()[i] = bitstream.read_u8(6);
            }

            configuration.trueskill_sigma_multiplier = bitstream.read_float(32);
            configuration.trueskill_beta_performance_variation = bitstream.read_float(32);
            configuration.trueskill_tau_dynamics_factor = bitstream.read_float(32);
            configuration.trueskill_adjust_tau_with_update_weight = bitstream.read_bool();
            configuration.trueskill_draw_probability = bitstream.read_u8(7);
            configuration.trueskill_hillclimb_w0 = bitstream.read_u8(7);
            configuration.trueskill_hillclimb_w50 = bitstream.read_u8(7);
            configuration.trueskill_hillclimb_w100 = bitstream.read_u8(7);
            configuration.trueskill_hillclimb_w150 = bitstream.read_u8(7);
            configuration.skill_update_weight_s0 = bitstream.read_u8(7);
            configuration.skill_update_weight_s10 = bitstream.read_u8(7);
            configuration.skill_update_weight_s20 = bitstream.read_u8(7);
            configuration.skill_update_weight_s30 = bitstream.read_u8(7);
            configuration.skill_update_weight_s40 = bitstream.read_u8(7);
            configuration.skill_update_weight_s50 = bitstream.read_u8(7);
            configuration.quality_update_weight_q0 = bitstream.read_u8(7);
            configuration.quality_update_weight_q25 = bitstream.read_u8(7);
            configuration.quality_update_weight_q50 = bitstream.read_u8(7);
            configuration.quality_update_weight_q75 = bitstream.read_u8(7);
            configuration.quality_update_weight_q100 = bitstream.read_u8(7);

            if configuration.hopper_type <= 1 {
                configuration.minimum_player_count = bitstream.read_u8(4) + 1;
                configuration.maximum_player_count = bitstream.read_u8(4) + 1;
            }
            else if configuration.hopper_type >= 2 {
                configuration.team_count = bitstream.read_u8(3) + 1;
                configuration.minimum_team_size = bitstream.read_u8(3) + 1;
                configuration.maximum_team_size = bitstream.read_u8(3) + 1;

                if configuration.hopper_type == 2 {
                    configuration.allow_uneven_teams = bitstream.read_bool();
                    configuration.allow_parties_to_split = bitstream.read_bool();
                }
                else if configuration.hopper_type == 3   {
                    configuration.maximum_team_imbalance = bitstream.read_u8(3);
                    configuration.big_squad_size_threshold = bitstream.read_u8(4) + 1;
                    configuration.maximum_big_squad_imbalance = bitstream.read_u8(3);
                    configuration.enable_big_squad_mixed_skill_restrictions = bitstream.read_bool();
                }
            }
        }
    }
}