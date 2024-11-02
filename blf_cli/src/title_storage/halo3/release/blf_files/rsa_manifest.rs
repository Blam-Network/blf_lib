use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_map_manifest, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;

pub const k_rsa_manifest_file_name: &str = "rsa_manifest.bin";


blf_file! {
    pub struct rsa_manifest {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mapm: s_blf_chunk_map_manifest,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl rsa_manifest {
    pub fn create(mapm: &s_blf_chunk_map_manifest) -> Self {
        Self {
            _blf: s_blf_chunk_start_of_file::default(),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            mapm: mapm.clone(),
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}