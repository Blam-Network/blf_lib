#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::*;
use blf_lib::blf::{BlfFile, BlfFileBuilder};
use blf_lib::blf::chunks::find_chunk_in_file;
use blf_lib::derive::BlfFile;
use blf_lib::blf::chunks::SerializableBlfChunk;
use blf_lib::types::build_number_identifier::build_number_identifier;
use blf_lib::types::byte_order_mark::byte_order_mark;

#[derive(BlfFile)]
struct test_blf_file {
    _blf: s_blf_chunk_start_of_file,
    athr: s_blf_chunk_author,
    _eof: s_blf_chunk_end_of_file,
}

fn main() {
    let mut blf_file = test_blf_file {
        _blf: s_blf_chunk_start_of_file::new(
            "Test BLF File 1",
            byte_order_mark::default()
        ),
        athr: s_blf_chunk_author::new(
            "",
            build_number_identifier::new(1, 12070),
            "12070.08.09.05.2031.halo3_ship",
            "blf_lib"
        ),
        _eof: s_blf_chunk_end_of_file::default(),
    };

    blf_file.write("C:\\Users\\stell\\Desktop\\test_3.bin");

    let mut blf_file_builder = BlfFileBuilder::new();
    let blf_file_2 = blf_file_builder
        .add_chunk(s_blf_chunk_start_of_file::new(
            "Test BLF File 2",
            byte_order_mark::default()
        ))
        .add_chunk(s_blf_chunk_author::new(
            "",
            build_number_identifier::new(1, 12070),
            "12070.08.09.05.2031.halo3_ship",
            "blf_lib"
        ))
        .add_chunk(s_blf_chunk_end_of_file::default());

    blf_file_2.write("C:\\Users\\stell\\Desktop\\test_2.bin");

    // TODO: Fix read
    let _blf = find_chunk_in_file::<s_blf_chunk_start_of_file>("C:\\Users\\stell\\Desktop\\test_3.bin").unwrap();
    let athr = find_chunk_in_file::<s_blf_chunk_author>("C:\\Users\\stell\\Desktop\\test_3.bin").unwrap();
    let _eof = find_chunk_in_file::<s_blf_chunk_end_of_file>("C:\\Users\\stell\\Desktop\\test_3.bin").unwrap();

    let mut blf_file_builder_2 = BlfFileBuilder::new();
    blf_file_builder_2.add_chunk(_blf);
    blf_file_builder_2.add_chunk(athr);
    blf_file_builder_2.add_chunk(_eof);
    blf_file_builder_2.write("C:\\Users\\stell\\Desktop\\test_4.bin");
}