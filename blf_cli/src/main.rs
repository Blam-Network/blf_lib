#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod title_storage;
mod io;

use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::*;
use blf_lib::derive::BlfFile;
use blf_lib::blf::chunks::SerializableBlfChunk;

#[derive(BlfFile, Default)]
struct test_blf_file {
    _blf: s_blf_chunk_start_of_file,
    athr: s_blf_chunk_author,
    motd: s_blf_chunk_message_of_the_day,
    _eof: s_blf_chunk_end_of_file,
}

fn main() {
    let mut title_converter =
        title_storage::get_title_converter("Halo 3".to_string(), "12070.08.09.05.2031.halo3_ship".to_string()).unwrap();

    title_converter.build_config(
        &"C:\\Users\\stell\\Downloads\\RawGames-Halo\\RawGames-Halo\\Halo 3\\11855.07.08.20.2317.halo3_ship\\title storage\\title2".to_string(),
        &"C:\\Users\\stell\\Desktop\\Blam-Title-Storage\\Halo 3\\Release".to_string()
    );
}