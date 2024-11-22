pub(crate) mod mcc;

use std::fs::{exists, File};
use std::io::Read;
use std::str::FromStr;
use blf_lib::blam::common::memory::crc::crc32;
use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::{c_map_variant, s_variant_object_datum, s_variant_quota};
use blf_lib::blf::chunks::search_for_chunk_in_file;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_game_variant, s_blf_chunk_map_variant, s_blf_chunk_packed_game_variant, s_blf_chunk_packed_map_variant};
use crate::{build_path, debug_log};
use crate::console::console_task;
use crate::io::get_files_in_folder;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::config_rsa_signature_file_map_id_regex;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::variant_importer::mcc::object_indexes::get_h3_index_for_mcc_object;
use crate::title_storage::halo3::release::blf_files::map_variant::get_map_budget;

pub fn import_variant(hoppers_config_path: &String, variant_path: &String) {
    let mut task = console_task::start("Importing Variant");

    let mut game_variant: Option<c_game_variant> = None;
    let game_variant_chunk = search_for_chunk_in_file::<s_blf_chunk_game_variant>(variant_path);
    if game_variant_chunk.is_some() {
        game_variant = Some(game_variant_chunk.unwrap().game_variant);
    }
    let packed_game_variant_chunk = search_for_chunk_in_file::<s_blf_chunk_packed_game_variant>(variant_path);
    if packed_game_variant_chunk.is_some() {
        game_variant = Some(packed_game_variant_chunk.unwrap().game_variant);
    }
    if game_variant.is_some() {
        let game_variant = game_variant.unwrap();
        let output_file_name = format!("{}.json", game_variant.m_base_variant.m_metadata.name.get_string()
            .replace(" ", "_")
            .to_lowercase());
        let output_file = File::create(build_path!(
            hoppers_config_path,
            "game_variants",
            &output_file_name
        )).unwrap();
        serde_json::to_writer_pretty(output_file, &game_variant.clone()).unwrap();

        task.add_message(format!("Added game variant: {output_file_name}"));
        task.complete();
        return;
    }

    let mut map_variant: Option<c_map_variant> = None;
    let map_variant_chunk = search_for_chunk_in_file::<s_blf_chunk_map_variant>(variant_path);
    if map_variant_chunk.is_some() {
        map_variant = Some(map_variant_chunk.unwrap().map_variant);
    }
    let packed_map_variant = search_for_chunk_in_file::<s_blf_chunk_packed_map_variant>(variant_path);
    if packed_map_variant.is_some() {
        map_variant = Some(packed_map_variant.unwrap().map_variant);
    }
    if map_variant.is_some() {
        let mut map_variant = map_variant.unwrap().clone();

        if map_variant.m_map_variant_version == 13 || map_variant.m_map_variant_version == 14 {
            convert_mcc_map(&mut task, hoppers_config_path, &mut map_variant);
        }

        let output_file_name = format!("{}.json", map_variant.m_metadata.name.get_string()
            .replace(" ", "_")
            .to_lowercase());

        let output_file = File::create(build_path!(
            hoppers_config_path,
            "map_variants",
            &output_file_name
        )).unwrap();

        serde_json::to_writer_pretty(output_file, &map_variant.clone()).unwrap();

        task.add_message(format!("Added map variant: {output_file_name}"));
        task.complete();
        return;
    }

    task.fail_with_error("Unable to parse variant file.");
}

fn convert_mcc_map(task: &mut console_task, hoppers_config_folder: &String, map: &mut c_map_variant) {
    task.add_warning(format!("Version {} MCC map detected, conversion may be lossy.", map.m_map_variant_version));

    let mut bad_budget_indices = Vec::<i32>::new();
    for i in (0..map.m_number_of_placeable_object_quotas as usize).rev() {
        let quota = map.m_quotas.get_mut()[i];
        let h3_object_index = get_h3_index_for_mcc_object(map.m_map_id, quota.object_definition_index);

        if h3_object_index.is_some() {
            map.m_quotas.get_mut()[i].object_definition_index = h3_object_index.unwrap();
        }
        else {
            map.m_quotas.get_mut().remove(i);
            map.m_quotas.get_mut().push(s_variant_quota::default()); // Add an empty one to the end TODO: Refactor
            map.m_number_of_placeable_object_quotas -= 1;
            bad_budget_indices.push(i as i32);
        }
    }

    debug_log!("{} objects were removed from the budget.", bad_budget_indices.len());

    let mut removed_objects_count = 0;
    for i in (0..map.m_number_of_variant_objects as usize).rev() {
        if bad_budget_indices.contains(&map.m_variant_objects.get()[i].variant_quota_index) {
            map.m_variant_objects.get_mut().remove(i);
            map.m_variant_objects.get_mut().push(s_variant_object_datum::default());
            map.m_number_of_variant_objects -= 1;
            removed_objects_count += 1;
        }
    }
    if removed_objects_count > 0 {
        task.add_warning(format!("{} objects have been removed.", removed_objects_count));
    }

    // Realign indexes after removing from quotas.
    for bad_index in bad_budget_indices {
        for object_index in 0..map.m_number_of_variant_objects as usize {
            if map.m_variant_objects.get()[object_index].variant_quota_index > bad_index {
                map.m_variant_objects.get_mut()[object_index].variant_quota_index -= 1;
            }
        }
    }

    if map.m_spent_budget > get_map_budget(map.m_map_id) {
        task.add_warning(format!(
            "The spent budget ${} exceeds the limit ${}, some objects may not spawn.",
            map.m_spent_budget,
            get_map_budget(map.m_map_id)
        ));
    }

    map.m_map_variant_version = 12;

    // Update the checksum
    let rsa_folder = build_path!(
        hoppers_config_folder,
        "rsa_signatures"
    );

    if !exists(&rsa_folder).unwrap() {
        task.add_warning(format!("Couldn't find an RSA signature for map {}, the map checksum will be incorrect.", map.m_map_id));
    }

    let rsa_files = get_files_in_folder(&rsa_folder).unwrap_or_else(|err|{
        panic!();
    });

    for rsa_file_name in rsa_files {
        let map_id = config_rsa_signature_file_map_id_regex.captures(rsa_file_name.as_str()).unwrap();
        let map_id = map_id.get(0).unwrap();
        let map_id = u32::from_str(map_id.as_str()).unwrap();
        if map_id != map.m_map_id {
            continue;
        }

        let rsa_file_path = build_path!(
            &rsa_folder,
            &rsa_file_name
        );

        let rsa_file = File::open(&rsa_file_path);
        if rsa_file.is_err() {
            continue;
        }

        let mut rsa_file = rsa_file.unwrap();
        let mut rsa_signature = Vec::<u8>::with_capacity(0x100);
        rsa_file.read_to_end(&mut rsa_signature).unwrap();

        map.m_map_variant_checksum = crc32(0xFFFFFFFF, &rsa_signature);
        return;
    }

    task.add_warning(format!("Couldn't find an RSA signature for map {}, the map checksum will be incorrect.", map.m_map_id));
}

