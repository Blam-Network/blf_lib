use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_game_set, s_blf_chunk_game_set_entry, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;
use blf_lib::types::c_string::StaticString;
use crate::build_path;
use crate::io::create_parent_folders;
use std::fs::File;

pub const k_game_set_blf_file_name: &str = "game_set_006.bin";
pub const k_game_set_config_file_name: &str = "game_set.csv";

blf_file! {
    pub struct game_set {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        gset: s_blf_chunk_game_set,
        _eof: s_blf_chunk_end_of_file,
    }
}

#[derive(Serialize, Deserialize)]
pub struct game_set_config_row {
    pub map_variant_file_name: String,
    pub game_variant_file_name: String,
    pub weight: u32,
    pub minimum_player_count: u8,
    pub skip_after_veto: bool,
    pub optional: bool,
}

pub struct game_set_config {
    pub entries: Vec<game_set_config_row>,
}

impl game_set_config {
    pub fn read(path: String) -> Result<game_set_config, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new().from_path(&path).unwrap();
        let mut rows = Vec::<game_set_config_row>::new();
        for row in reader.deserialize() {
            if let Ok(row) = row {
                let row: game_set_config_row = row;
                rows.push(row);
            } else {
                return Err(Box::from(format!("Failed to parse game set CSV: {path}")));
            }
        }

        Ok(game_set_config { entries: rows })
    }
}

impl game_set {
    pub fn create(game_set: s_blf_chunk_game_set) -> game_set {
        game_set {
            _blf: s_blf_chunk_start_of_file::new("game set", byte_order_mark::default()),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            gset: game_set,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }

    pub fn write_to_config(&self, hopper_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut writer = WriterBuilder::new().from_writer(vec![]);
        let game_set = &self.gset;

        game_set.get_entries().iter().for_each(|game_entry| {
            writer.serialize(game_set_config_row {
                map_variant_file_name: game_entry.map_variant_file_name.get_string(),
                game_variant_file_name: game_entry.game_variant_file_name.get_string(),
                weight: game_entry.weight,
                minimum_player_count: game_entry.minimum_player_count,
                skip_after_veto: game_entry.skip_after_veto,
                optional: game_entry.optional,
            }).unwrap()
        });

        let game_set_config_path = build_path!(
            hopper_config_path,
            k_game_set_config_file_name
        );

        create_parent_folders(&game_set_config_path)?;

        let mut config_file = File::create(game_set_config_path)?;
        config_file.write_all(&writer.into_inner()?)?;

        Ok(())
    }

    pub fn create_from_config(
        config: &game_set_config,
        game_variant_hashes: &HashMap<String, s_network_http_request_hash>,
        map_variant_hashes: &HashMap<String, s_network_http_request_hash>,
        map_variant_map_ids: &HashMap<String, u32>,
    ) -> Result<game_set, Box<dyn Error>> {
        let mut gset_chunk = s_blf_chunk_game_set::default();
        for row in config.entries.iter() {
            gset_chunk.add_entry(s_blf_chunk_game_set_entry {
                map_variant_file_name: StaticString::from_string(&row.map_variant_file_name)?,
                game_variant_file_name: StaticString::from_string(&row.game_variant_file_name)?,
                weight: row.weight,
                minimum_player_count: row.minimum_player_count,
                skip_after_veto: row.skip_after_veto,
                optional: row.optional,

                map_variant_file_hash: map_variant_hashes.get(&row.map_variant_file_name)
                    .expect(&format!("No map variant hash found for {}", row.map_variant_file_name))
                    .clone(),
                game_variant_file_hash: game_variant_hashes.get(&row.game_variant_file_name)
                    .expect(&format!("No map variant hash found for {}", row.game_variant_file_name))
                    .clone(),
                map_id: map_variant_map_ids.get(&row.map_variant_file_name)
                    .expect(&format!("No map ID found for {}", row.map_variant_file_name))
                    .clone(),
            })?;
        }

        Ok(Self::create(gset_chunk))
    }
}