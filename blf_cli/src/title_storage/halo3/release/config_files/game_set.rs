use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_game_set;

#[derive(Serialize, Deserialize)]
pub struct game_set_row {
    pub map_variant_file_name: String,
    pub game_variant_file_name: String,
    pub weight: u32,
    pub minimum_player_count: u8,
    pub skip_after_veto: bool,
    pub optional: bool,
}


pub fn build_game_set_csv(game_set: &s_blf_chunk_game_set) -> String {
    let mut writer = WriterBuilder::new().from_writer(vec![]);

    game_set.game_entries.iter().for_each(|game_entry| {
        writer.serialize(game_set_row {
            map_variant_file_name: game_entry.map_variant_file_name.get_string(),
            game_variant_file_name: game_entry.game_variant_file_name.get_string(),
            weight: game_entry.weight,
            minimum_player_count: game_entry.minimum_player_count,
            skip_after_veto: game_entry.skip_after_veto,
            optional: game_entry.optional,
        }).unwrap()
    });

    String::from_utf8(writer.into_inner().unwrap()).unwrap()
}