use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_content_header, s_blf_chunk_end_of_file, s_blf_chunk_game_variant, s_blf_chunk_map_variant, s_blf_chunk_start_of_file};
use blf_lib::io::read_json_file;
use crate::console::console_task;

pub fn export_variant(source_json_path: &str, desination_path: &str) {
    let mut task = console_task::start("Exporting Variant");

    let mut blf_file = BlfFileBuilder::new();
    blf_file.add_chunk(s_blf_chunk_start_of_file::default());

    let map_variant = read_json_file::<c_map_variant>(source_json_path);
    let game_variant = read_json_file::<c_game_variant>(source_json_path);

    if map_variant.is_ok() {
        let map_variant = map_variant.unwrap();
        blf_file.add_chunk(s_blf_chunk_content_header::create_for_map_variant(&map_variant));
        blf_file.add_chunk(s_blf_chunk_map_variant::create(map_variant));
    }
    else if game_variant.is_ok() {
        let game_variant = game_variant.unwrap();
        blf_file.add_chunk(s_blf_chunk_content_header::create_for_game_variant(&game_variant));
        blf_file.add_chunk(s_blf_chunk_game_variant::create(game_variant));
    }
    else {
        task.fail_with_error("Unrecognized variant file type.");
    }

    blf_file.add_chunk(s_blf_chunk_end_of_file::default());

    blf_file.write_file(desination_path);

    task.complete();
}