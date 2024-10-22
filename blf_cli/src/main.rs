#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::{remove_file, File};
use std::io::{Read, Write};
use clap::{command, Parser};
use blf_lib::io::bitstream::{c_bitstream_writer, e_bitstream_byte_order};
use blf_lib::blf::chunks::find_chunk_in_file;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_map_variant, s_blf_chunk_packed_map_variant};
use crate::commands::Commands;
use crate::commands::import_rsa_signature::import_rsa_signature;
use crate::title_storage::halo3::release::blf_files::map_variant::map_variant;
use blf_lib::blf::BlfFile;

mod title_storage;
mod io;
mod console;
mod commands;

#[derive(Debug, Parser)]
#[command(name = "blf_cli")]
#[command(about = "blam! engine file editor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let packed_map_variant = find_chunk_in_file::<s_blf_chunk_packed_map_variant>("C:\\Users\\stell\\Desktop\\bitstream_test\\sandbox_utahmambo_012.bin").unwrap();
    println!("{:?}", packed_map_variant);

    // let map_variant = find_chunk_in_file::<s_blf_chunk_map_variant>("C:\\Users\\stell\\Desktop\\bitstream_test\\utahmambo_mapv.bin").unwrap();
    // let map_json = serde_json::to_string_pretty(&map_variant).unwrap();
    // let mut json_file = File::create("C:\\Users\\stell\\Desktop\\bitstream_test\\utahmambo.json").unwrap();
    //
    // json_file.write_all(map_json.as_bytes()).unwrap();
    //
    // return;

    // let mut map_json_file = File::open("C:\\Users\\stell\\Desktop\\bitstream_test\\utahmambo.json").unwrap();
    // let mut map_json: String = String::new();
    // map_json_file.read_to_string(&mut map_json).unwrap();
    // let mapv  = serde_json::from_str::<s_blf_chunk_map_variant>(&map_json).unwrap();
    // let mut data = [0u8; 0xE090]; // size of the map
    // let mut bitstream = c_bitstream_writer::new(&mut data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
    // bitstream.begin_writing(1);
    // mapv.map_variant.encode(&mut bitstream);
    //
    // let mut map_variant = map_variant::create(mapv.map_variant);
    // map_variant.write(&String::from("C:\\Users\\stell\\Desktop\\bitstream_test\\utahmambo_out_012.bin"));
    //
    // let mut data_length: usize = 0;
    // let mut remaining: usize = 0;
    // bitstream.finish_writing(&mut remaining);
    // let data = bitstream.get_data(&mut data_length);
    // let data = &data[0..data_length];
    // remove_file("C:\\Users\\stell\\Desktop\\bitstream_test\\death_valley_out_012.bin");
    // let mut map_blf_file = File::create_new("C:\\Users\\stell\\Desktop\\bitstream_test\\death_valley_out_012.bin").unwrap();
    // map_blf_file.write_all(&data).unwrap();

    // remove_file("C:\\Users\\stell\\Desktop\\bitstream_test\\atlas_actuality-break_012_new8.bin");
    // let mut map_blf_file = File::create_new("C:\\Users\\stell\\Desktop\\bitstream_test\\atlas_actuality-break_012_new8.bin").unwrap();
    // let mut map_blf_file_buffer = Vec::<u8>::new();
    // let mut data = [0u8; 0x2F94];
    // let mut bitstream = c_bitstream::new(&mut data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
    // bitstream.begin_writing(1);
    // bitstream.write_qword(0, 64);
    // bitstream.write_string_wchar(&String::from("Borderline"), 32);
    // bitstream.write_string_utf8(&String::from("The ancient readings suggest that danger lurks around every corner. 6-12 players"), 128);
    // bitstream.write_string_utf8(&String::from("Mini Waz"), 16);
    // bitstream.write_integer(11, 5);
    // bitstream.write_integer(1, 1);
    // bitstream.write_qword(0x000900000038C863 , 64);
    // bitstream.write_qword(57840, 64);
    //
    // let mut data_length: usize = 0;
    // let mut remaining: usize = 0;
    // bitstream.finish_writing(&mut remaining);
    // let data = bitstream.get_data(&mut data_length);
    // let data = &data[0..data_length];
    // map_blf_file.write_all(&data).unwrap();
    //
    //
    // return;
    // map_blf_file.seek_relative(0x8C); // skip _blf, athr, mapv header.
    // map_blf_file.read_to_end(&mut map_blf_file_buffer).unwrap();
    // s_blf_header::decode(map_blf_file_buffer.)

    //
    // let mut json_file = File::create("C:\\Users\\stell\\Desktop\\bitstream_test\\borderline.json").unwrap();
    //
    // json_file.write_all(map_json.as_bytes()).unwrap();
    // println!("{:?}", map_variant);

    return;

    let args = Cli::parse();

    match args.command {
        Commands::BuildTitleStorage { config_input_path, blf_output_path, title, version } => {
            let mut title_converter =
                title_storage::get_title_converter(title, version)
                    .expect("No title converter was found for the provided title and version.");

            title_converter.build_blfs(
                &config_input_path,
                &blf_output_path
            );
        },
        Commands::BuildTitleStorageConfig { blf_input_path, config_output_path, title, version } => {
            let mut title_converter =
                title_storage::get_title_converter(title, version)
                    .expect("No title converter was found for the provided title and version.");

            title_converter.build_config(
                &blf_input_path,
                &config_output_path
            );
        },
        Commands::ImportRsaSignature { config_path, map_file_path, title, version } => {
            import_rsa_signature(config_path, map_file_path, title, version);
        }
    }
}