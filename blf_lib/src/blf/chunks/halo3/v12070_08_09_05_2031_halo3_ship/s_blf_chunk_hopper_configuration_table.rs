use libc::time_t;
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
    pub category_image_index: u16,
}

pub struct c_hopper_configuration {
    hopper_name: StaticString<32>,
    game_set_hash: s_network_http_request_hash,
    hopper_identifier: u16,
    hopper_category: u32,
    hopper_type: u32,
    image_index: u32,
    xlast_index: u32,
    rich_presence_id: u8,
    // __data49[0x7];
    start_time: time_t,
    end_time: time_t,
    hopper_regions: u32,
    minimum_base_xp: u32,
    maximum_base_xp: u32,
    minimum_games_played: u32,
    maximum_games_played: u32,
    minimum_party_size: u32,
    maximum_party_size: u32,
    hopper_access_bit: u32,
    account_type_access: u32,
    require_all_party_members_meet_games_played_requirements: bool,
    require_all_party_members_meet_experience_requirements: bool,
    require_all_party_members_meet_access_requirements: bool,
    require_all_party_members_meet_live_account_access_requirements: bool,
    hide_hopper_from_games_played_restricted_players: bool,
    hide_hopper_from_experience_restricted_players: bool,
    hide_hopper_from_access_restricted_players: bool,
    hide_hopper_from_live_account_access_restricted_players: bool,
    hide_hopper_due_to_time_restriction: bool,
    requires_all_downloadable_maps: bool,
    veto_enabled: bool,
    guests_allowed: bool,
    require_hosts_on_multiple_teams: bool,
    stats_write: u32,
    language_filter: u32,
    country_code_filter: u32,
    gamerzone_filter: u32,
    quitter_filter_percentage: u32,
    quitter_filter_maximum_party_size: u32,
    rematch_countdown_timer: u32,
    rematch_group_formation: u32,
    repeated_opponents_to_consider_for_penalty: u32,
    repeated_opponents_experience_threshold: u32,
    repeated_opponents_skill_throttle_start: u32,
    repeated_opponents_skill_throttle_stop: u32,
    maximum_total_matchmaking_seconds: u32,
    gather_start_game_early_seconds: u32,
    gather_give_up_seconds: u32,
    chance_of_gathering: [u8;16],
    experience_points_per_win: u32,
    experience_penalty_per_drop: u32,
    minimum_mu_per_level: [f32; 49],
    maximum_skill_level_match_delta: [u8; 50],
    trueskill_sigma_multiplier: f32,
    trueskill_beta_performance_variation: f32,
    trueskill_tau_dynamics_factor: f32,
    trueskill_adjust_tau_with_update_weight: bool,
    trueskill_draw_probability: u32,
    trueskill_hillclimb_w0: u32,
    trueskill_hillclimb_w50: u32,
    trueskill_hillclimb_w100: u32,
    trueskill_hillclimb_w150: u32,
    skill_update_weight_s0: u32,
    skill_update_weight_s10: u32,
    skill_update_weight_s20: u32,
    skill_update_weight_s30: u32,
    skill_update_weight_s40: u32,
    skill_update_weight_s50: u32,
    quality_update_weight_q0: u32,
    quality_update_weight_q25: u32,
    quality_update_weight_q50: u32,
    quality_update_weight_q75: u32,
    quality_update_weight_q100: u32,
    pre_match_voice: u32,
    in_match_voice: u32,
    post_match_voice: u32,
    restrict_open_channel: bool,
}

pub const k_hopper_maximum_hopper_count: usize = 32;

blf_chunk!(
    #[Signature("mhcf")]
    #[Version(11.1)]
    pub struct s_blf_chunk_hopper_configuration_table
    {
        hopper_category_count: u8,
        hopper_category: StaticArray<s_game_hopper_custom_category, 4>,

        hopper_configuration_count: u8,
        

    }
);

impl SerializableBlfChunk for s_blf_chunk_hopper_configuration_table {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
        let mut bitstream = create_bitstream_writer(0x1BC0, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        bitstream.write_integer(self.game_entry_count as u32, 6);

        for i in 0..self.game_entry_count {
            let game_entry = self.game_entries[i];
            bitstream.write_integer(game_entry.weight, 32);
            bitstream.write_integer(game_entry.minimum_player_count as u32, 4);
            bitstream.write_bool(game_entry.skip_after_veto);
            bitstream.write_bool(game_entry.optional);
            bitstream.write_integer(game_entry.map_id, 32);
            bitstream.write_string_utf8(&game_entry.game_variant_file_name.get_string(), 32);
            bitstream.write_raw_data(&game_entry.game_variant_file_hash.data, 0x100);
            bitstream.write_string_utf8(&game_entry.map_variant_file_name.get_string(), 32);
            bitstream.write_raw_data(&game_entry.map_variant_file_hash.data, 0x100);
        }

        close_bitstream_writer(&mut bitstream)
    }

    fn decode_body(&mut self, buffer: &[u8]) {
        let mut bitstream = create_bitstream_reader(buffer, e_bitstream_byte_order::_bitstream_byte_order_big_endian);

        self.game_entry_count = bitstream.read_integer(6) as usize;
        self.game_entries.resize(self.game_entry_count, s_blf_chunk_game_set_entry::default());

        for i in 0..self.game_entry_count {
            let game_entry = &mut self.game_entries.as_mut_slice()[i];
            game_entry.weight = bitstream.read_integer(32);
            game_entry.minimum_player_count = bitstream.read_integer(4) as u8;
            game_entry.skip_after_veto = bitstream.read_bool();
            game_entry.optional = bitstream.read_bool();
            game_entry.map_id = bitstream.read_integer(32);
            game_entry.game_variant_file_name.set_string(&bitstream.read_string_utf8(32)).unwrap();
            game_entry.game_variant_file_hash = s_network_http_request_hash::try_from(bitstream.read_raw_data(0xA0)).unwrap();
            game_entry.map_variant_file_name.set_string(&bitstream.read_string_utf8(32)).unwrap();
            game_entry.map_variant_file_hash = s_network_http_request_hash::try_from(bitstream.read_raw_data(0xA0)).unwrap();
        }
    }
}