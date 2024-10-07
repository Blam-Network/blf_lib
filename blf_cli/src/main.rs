#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use blf_lib::blam::memory::bitstream::c_bitstream;
use blf_lib::blf::s_blf_header::s_blf_header;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::*;
use blf_lib::blf::versions::version_factory;

fn main() {
    let mut my_shitty_data = [0x5F, 0x62, 0x6C, 0x66, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02, 0xFF, 0xFE];

    let mut header: s_blf_header = s_blf_header::default();
    let mut _blf: s_blf_chunk_start_of_file = s_blf_chunk_start_of_file::default();

    let mut bitstream: c_bitstream = c_bitstream::new(&mut my_shitty_data);
    bitstream.read_integer(32); // skip the signature
    header.chunk_size = bitstream.read_integer(32);
    header.major_version = bitstream.read_integer(16) as u16;
    header.minor_version = bitstream.read_integer(16) as u16;

    _blf.byte_order_mark = bitstream.read_integer(16) as u16;

    let version = version_factory::get_version("Halo 3", "12070.08.09.05.2031.halo3_ship");
}