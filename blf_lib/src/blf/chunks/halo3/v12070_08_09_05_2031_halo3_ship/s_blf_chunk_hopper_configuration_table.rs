use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::io::bitstream::{c_bitstream_reader, create_bitstream_writer, e_bitstream_byte_order};
use blf_lib::types::array::StaticArray;
use crate::types::c_string::StaticString;
use blf_lib::types::time::{filetime};
use crate::io::bitstream::close_bitstream_writer;
use serde_hex::{SerHex,StrictCap};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Signature("mhcf")]
#[Version(11.1)]
pub struct s_blf_chunk_hopper_configuration_table
{
    hopper_category_count: u8,
    hopper_categories: Vec<s_game_hopper_custom_category>,

    hopper_configuration_count: u8,
    hopper_configurations: Vec<c_hopper_configuration>,
}

impl BlfChunkHooks for s_blf_chunk_hopper_configuration_table {}

#[derive(Clone, Default, PartialEq, Debug, Copy, Serialize, Deserialize)]
pub struct s_game_hopper_custom_category {
    pub category_identifier: u16,
    pub category_image_index: u8,
    pub category_name: StaticString<15>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct c_hopper_configuration {
    pub hopper_name: StaticString<15>,
    pub game_set_hash: s_network_http_request_hash,
    pub hopper_identifier: u16,
    pub hopper_category: u16,
    pub hopper_type: u8,
    pub image_index: u8,
    pub xlast_index: u8,
    pub rich_presence_id: u16,
    // __data49[0x7];
    pub start_time: filetime,
    pub end_time: filetime,
    #[serde(with = "SerHex::<StrictCap>")]
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

impl s_blf_chunk_hopper_configuration_table {
    pub fn get_hopper_categories(&self) -> Vec<s_game_hopper_custom_category> {
        self.hopper_categories.as_slice()[0..self.hopper_category_count as usize].to_vec()
    }

    pub fn get_hopper_configurations(&self) -> Vec<c_hopper_configuration> {
        self.hopper_configurations.as_slice()[0..self.hopper_configuration_count as usize].to_vec()
    }

    pub fn add_hopper_configuration(&mut self, config: c_hopper_configuration) -> Result<(), String> {
        if self.hopper_configuration_count as usize >= k_hopper_maximum_hopper_count {
            return Err("The hopper config chunk is full!".to_string());
        }
        self.hopper_configuration_count += 1;
        self.hopper_configurations.push(config);
        Ok(())
    }

    pub fn add_category_configuration(&mut self, config: s_game_hopper_custom_category) -> Result<(), String> {
        if self.hopper_category_count as usize >= 4 {
            return Err("The hopper config chunk is full!".to_string());
        }
        self.hopper_category_count += 1;
        self.hopper_categories.push(config);
        Ok(())
    }

    pub fn hopper_configuration_count(&self) -> usize {
        self.hopper_configuration_count as usize
    }

}


impl BinRead for s_blf_chunk_hopper_configuration_table {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)?;

        let mut bitstream = c_bitstream_reader::new(buffer.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();

        let mut mhcf = Self::default();


        mhcf.hopper_category_count = bitstream.read_u8(3);
        mhcf.hopper_categories.resize(mhcf.hopper_category_count as usize, s_game_hopper_custom_category::default());

        for i in 0..mhcf.hopper_category_count as usize {
            let category = &mut mhcf.hopper_categories[i];
            category.category_identifier = bitstream.read_u16(16);
            category.category_image_index = bitstream.read_u8(6);
            category.category_name.set_string(&bitstream.read_string_utf8(16)).unwrap();
        }

        mhcf.hopper_configuration_count = bitstream.read_u8(6);
        mhcf.hopper_configurations.resize(mhcf.hopper_configuration_count as usize, c_hopper_configuration::default());

        for i in 0..mhcf.hopper_configuration_count as usize {
            let configuration = &mut mhcf.hopper_configurations[i];
            configuration.hopper_name.set_string(&bitstream.read_string_utf8(16)).unwrap();
            configuration.game_set_hash = s_network_http_request_hash::try_from(bitstream.read_raw_data(0xA0)).unwrap();
            configuration.hopper_identifier = bitstream.read_u16(16);
            configuration.hopper_category = bitstream.read_u16(16);
            configuration.hopper_type = bitstream.read_u8(2);
            configuration.image_index = bitstream.read_u8(6);
            configuration.xlast_index = bitstream.read_u8(5);
            configuration.rich_presence_id = bitstream.read_u16(16);
            configuration.start_time = filetime::from_u64(bitstream.read_qword(64));
            configuration.end_time = filetime::from_u64(bitstream.read_qword(64));
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

        Ok(mhcf)
    }
}

impl BinWrite for s_blf_chunk_hopper_configuration_table {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let mut bitstream = create_bitstream_writer(0x4C98, e_bitstream_byte_order::_bitstream_byte_order_big_endian);


        // Encode hopper_category_count
        bitstream.write_integer(self.hopper_category_count as u32, 3);

        // Encode each hopper category
        for i in 0..self.hopper_category_count as usize {
            let category = &self.hopper_categories[i];
            bitstream.write_integer(category.category_identifier as u32, 16);
            bitstream.write_integer(category.category_image_index as u32, 6);
            bitstream.write_string_utf8(&category.category_name.get_string(), 16);
        }

        // Encode hopper_configuration_count
        bitstream.write_integer(self.hopper_configuration_count as u32, 6);

        // Encode each hopper configuration
        for i in 0..self.hopper_configuration_count as usize {
            let configuration = &self.hopper_configurations[i];
            bitstream.write_string_utf8(&configuration.hopper_name.get_string(), 16);
            bitstream.write_raw_data(&configuration.game_set_hash.data, 0xA0);
            bitstream.write_integer(configuration.hopper_identifier as u32, 16);
            bitstream.write_integer(configuration.hopper_category as u32, 16);
            bitstream.write_integer(configuration.hopper_type as u32, 2);
            bitstream.write_integer(configuration.image_index as u32, 6);
            bitstream.write_integer(configuration.xlast_index as u32, 5);
            bitstream.write_integer(configuration.rich_presence_id as u32, 16);
            bitstream.write_qword(configuration.start_time.as_u64(), 64);
            bitstream.write_qword(configuration.end_time.as_u64(), 64);
            bitstream.write_integer(configuration.hopper_regions, 32);
            bitstream.write_integer(configuration.minimum_base_xp, 17);
            bitstream.write_integer(configuration.maximum_base_xp, 17);
            bitstream.write_integer(configuration.minimum_games_played, 17);
            bitstream.write_integer(configuration.maximum_games_played, 17);
            bitstream.write_integer(configuration.minimum_party_size - 1, 4);
            bitstream.write_integer(configuration.maximum_party_size - 1, 4);
            bitstream.write_integer((configuration.hopper_access_bit as i32 + 1) as u32, 4);
            bitstream.write_integer(configuration.account_type_access as u32, 2);
            bitstream.write_bool(configuration.require_all_party_members_meet_games_played_requirements);
            bitstream.write_bool(configuration.require_all_party_members_meet_experience_requirements);
            bitstream.write_bool(configuration.require_all_party_members_meet_access_requirements);
            bitstream.write_bool(configuration.require_all_party_members_meet_live_account_access_requirements);
            bitstream.write_bool(configuration.hide_hopper_from_games_played_restricted_players);
            bitstream.write_bool(configuration.hide_hopper_from_experience_restricted_players);
            bitstream.write_bool(configuration.hide_hopper_from_access_restricted_players);
            bitstream.write_bool(configuration.hide_hopper_from_live_account_access_restricted_players);
            bitstream.write_bool(configuration.hide_hopper_due_to_time_restriction);
            bitstream.write_integer(configuration.pre_match_voice as u32, 2);
            bitstream.write_integer(configuration.in_match_voice as u32, 2);
            bitstream.write_integer(configuration.post_match_voice as u32, 2);
            bitstream.write_bool(configuration.restrict_open_channel);
            bitstream.write_bool(configuration.requires_all_downloadable_maps);
            bitstream.write_bool(configuration.veto_enabled);
            bitstream.write_bool(configuration.guests_allowed);
            bitstream.write_bool(configuration.require_hosts_on_multiple_teams);
            bitstream.write_integer(configuration.stats_write as u32, 2);
            bitstream.write_integer(configuration.language_filter as u32, 2);
            bitstream.write_integer(configuration.country_code_filter as u32, 2);
            bitstream.write_integer(configuration.gamerzone_filter as u32, 2);
            bitstream.write_integer(configuration.quitter_filter_percentage as u32, 7);
            bitstream.write_integer(configuration.quitter_filter_maximum_party_size as u32, 4);
            bitstream.write_integer(configuration.rematch_countdown_timer as u32, 10);
            bitstream.write_integer(configuration.rematch_group_formation as u32, 2);
            bitstream.write_integer(configuration.repeated_opponents_to_consider_for_penalty as u32, 7);
            bitstream.write_integer(configuration.repeated_opponents_experience_threshold as u32, 4);
            bitstream.write_integer(configuration.repeated_opponents_skill_throttle_start as u32, 4);
            bitstream.write_integer(configuration.repeated_opponents_skill_throttle_stop as u32, 4);
            bitstream.write_integer(configuration.maximum_total_matchmaking_seconds as u32, 10);
            bitstream.write_integer(configuration.gather_start_game_early_seconds as u32, 10);
            bitstream.write_integer(configuration.gather_give_up_seconds as u32, 10);

            for chance in &configuration.chance_of_gathering {
                bitstream.write_integer(*chance as u32, 7);
            }

            bitstream.write_integer(configuration.experience_points_per_win as u32, 2);
            bitstream.write_integer(configuration.experience_penalty_per_drop as u32, 2);

            for min_mu in configuration.minimum_mu_per_level.get().iter() {
                bitstream.write_float(*min_mu, 32);
            }

            for max_skill_delta in configuration.maximum_skill_level_match_delta.get().iter() {
                bitstream.write_integer(*max_skill_delta as u32, 6);
            }

            bitstream.write_float(configuration.trueskill_sigma_multiplier, 32);
            bitstream.write_float(configuration.trueskill_beta_performance_variation, 32);
            bitstream.write_float(configuration.trueskill_tau_dynamics_factor, 32);
            bitstream.write_bool(configuration.trueskill_adjust_tau_with_update_weight);
            bitstream.write_integer(configuration.trueskill_draw_probability as u32, 7);
            bitstream.write_integer(configuration.trueskill_hillclimb_w0 as u32, 7);
            bitstream.write_integer(configuration.trueskill_hillclimb_w50 as u32, 7);
            bitstream.write_integer(configuration.trueskill_hillclimb_w100 as u32, 7);
            bitstream.write_integer(configuration.trueskill_hillclimb_w150 as u32, 7);
            bitstream.write_integer(configuration.skill_update_weight_s0 as u32, 7);
            bitstream.write_integer(configuration.skill_update_weight_s10 as u32, 7);
            bitstream.write_integer(configuration.skill_update_weight_s20 as u32, 7);
            bitstream.write_integer(configuration.skill_update_weight_s30 as u32, 7);
            bitstream.write_integer(configuration.skill_update_weight_s40 as u32, 7);
            bitstream.write_integer(configuration.skill_update_weight_s50 as u32, 7);
            bitstream.write_integer(configuration.quality_update_weight_q0 as u32, 7);
            bitstream.write_integer(configuration.quality_update_weight_q25 as u32, 7);
            bitstream.write_integer(configuration.quality_update_weight_q50 as u32, 7);
            bitstream.write_integer(configuration.quality_update_weight_q75 as u32, 7);
            bitstream.write_integer(configuration.quality_update_weight_q100 as u32, 7);

            if configuration.hopper_type <= 1 {
                bitstream.write_integer((configuration.minimum_player_count - 1) as u32, 4);
                bitstream.write_integer((configuration.maximum_player_count - 1) as u32, 4);
            } else if configuration.hopper_type >= 2 {
                bitstream.write_integer((configuration.team_count - 1) as u32, 3);
                bitstream.write_integer((configuration.minimum_team_size - 1) as u32, 3);
                bitstream.write_integer((configuration.maximum_team_size - 1) as u32, 3);

                if configuration.hopper_type == 2 {
                    bitstream.write_bool(configuration.allow_uneven_teams);
                    bitstream.write_bool(configuration.allow_parties_to_split);
                } else if configuration.hopper_type == 3 {
                    bitstream.write_integer(configuration.maximum_team_imbalance as u32, 3);
                    bitstream.write_integer((configuration.big_squad_size_threshold - 1) as u32, 4);
                    bitstream.write_integer(configuration.maximum_big_squad_imbalance as u32, 3);
                    bitstream.write_bool(configuration.enable_big_squad_mixed_skill_restrictions);
                }
            }
        }
        writer.write_ne(&close_bitstream_writer(&mut bitstream))
    }
}
