use std::error::Error;
use std::fs::create_dir_all;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::{c_map_variant, s_variant_object_datum, s_variant_quota};
use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::chunks::search_for_chunk_in_file;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_content_header, s_blf_chunk_end_of_file, s_blf_chunk_map_variant, s_blf_chunk_packed_map_variant, s_blf_chunk_start_of_file};
use crate::build_path;
use crate::console::console_task;
use crate::io::get_files_in_folder;
use crate::title_storage::check_file_exists;
use crate::title_storage::halo3::release::blf_files::map_variant::get_map_budget;

pub fn convert_halo3mcc_map_variants(mcc_maps_folder: String, converted_maps_path: String) {
    let mut task = console_task::start(String::from("Converting MCC map variants to 360"));

    if !check_file_exists(&mcc_maps_folder) {
        task.fail_with_error("The MCC Maps folder could not be found.");
        return;
    }

    create_dir_all(&converted_maps_path).unwrap();

    let variant_file_names = get_files_in_folder(&mcc_maps_folder).unwrap_or_else(|err|{
        task.fail_with_error(err);
        panic!()
    });

    for variant_file_name in variant_file_names {
        let mut map_variant: Option<c_map_variant> = None;
        let variant_file_path = build_path!(
            &mcc_maps_folder,
            variant_file_name
        );
        let map_variant_chunk = search_for_chunk_in_file::<s_blf_chunk_map_variant>(&variant_file_path);
        if map_variant_chunk.is_some() {
            map_variant = Some(map_variant_chunk.unwrap().map_variant);
        }
        let packed_map_variant = search_for_chunk_in_file::<s_blf_chunk_packed_map_variant>(&variant_file_path);
        if packed_map_variant.is_some() {
            map_variant = Some(packed_map_variant.unwrap().map_variant);
        }
        if map_variant.is_some() {
            let mut map_variant = map_variant.unwrap().clone();

            if map_variant.m_map_variant_version == 13 || map_variant.m_map_variant_version == 14 {
                if !convert_mcc_map(&mut task, &mut map_variant) {
                    continue;
                }
            }

            let output_file_name = format!("{}.bin", map_variant.m_metadata.name.get_string());

            let mut blf_file = BlfFileBuilder::new();
            blf_file.add_chunk(s_blf_chunk_start_of_file::default());
            blf_file.add_chunk(s_blf_chunk_content_header::create_for_map_variant(&map_variant));
            blf_file.add_chunk(s_blf_chunk_map_variant::create(map_variant));
            blf_file.add_chunk(s_blf_chunk_end_of_file::default());

            blf_file.write_file(build_path!(
                &converted_maps_path,
                &output_file_name
            ));
        }
    }
    task.complete();
}

fn convert_mcc_map(task: &mut console_task, map: &mut c_map_variant) -> bool {
    let checksum = get_checksum_for_map(map.m_map_id);

    if checksum.is_err() {
        task.add_error(format!("Skipped map variant {} because the map is unsupported.", map.m_metadata.name.get_string()));
        return false;
    } else {
        map.m_original_map_rsa_signature_hash = checksum.unwrap();
    }

    let mut bad_budget_indices = Vec::<i32>::new();
    for i in (0..map.m_number_of_placeable_object_quotas as usize).rev() {
        let quota = map.m_quotas.get_mut()[i];
        let h3_object_index = crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::variant_importer::mcc::object_indexes::get_h3_index_for_mcc_object(map.m_map_id, quota.object_definition_index);

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
        task.add_warning(format!("{}: {} objects have been removed.", map.m_metadata.name.get_string(), removed_objects_count));
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
            "{}: The spent budget ${} exceeds the limit ${}, some objects may not spawn.",
            map.m_metadata.name.get_string(),
            map.m_spent_budget,
            get_map_budget(map.m_map_id)
        ));
    }

    map.m_map_variant_version = 12;

    task.add_message(format!("Converted map {}", map.m_metadata.name.get_string()));

    true
}

fn get_checksum_for_map(map_id: u32) -> Result<u32, Box<dyn Error>> {
    match map_id {
        030 => Ok(0xA9494AE8), // Last Resort
        300 => Ok(0x62C9F673), // Construct
        310 => Ok(0xC9786BEC), // High Ground
        320 => Ok(0xBD822912), // Guardian
        330 => Ok(0x0F13C989), // Isolation
        340 => Ok(0xF1A889B8), // Valhalla
        350 => Ok(0x102B6C7A), // Epitaph
        360 => Ok(0xBF08D3D8), // Snowbound
        380 => Ok(0x5490CC8F), // Narrows
        390 => Ok(0x17EA5C32), // The Pit
        400 => Ok(0x3375A6F1), // Sandtrap
        410 => Ok(0x23ADA720), // Standoff
        440 => Ok(0xBCAFBE41), // Longshore
        470 => Ok(0x1CC05515), // Avalanche
        480 => Ok(0x65DC145C), // Foundry
        490 => Ok(0x55478205), // Assembly
        500 => Ok(0xF025A6FA), // Orbital
        520 => Ok(0x31DA07AB), // Blackout
        580 => Ok(0xF1197F82), // Rats Nest
        590 => Ok(0x2915FE0F), // Ghost Town
        600 => Ok(0xA468CD10), // Cold Storage
        720 => Ok(0x6B465319), // Heretic
        730 => Ok(0x59A2A65C), // Sandbox
        740 => Ok(0x6616ACC9), // Citadel
        _ => {
            Err(Box::from(format!("No checksum was found for map ID {map_id}")))
        }
    }
}