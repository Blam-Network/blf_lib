use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_end_of_file, s_blf_chunk_online_file_manifest, s_blf_chunk_start_of_file};
use blf_lib::blf_file;

pub const k_manifest_file_name: &str = "manifest_001.bin";

blf_file! {
    pub struct manifest {
        _blf: s_blf_chunk_start_of_file,
        onfm: s_blf_chunk_online_file_manifest,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl manifest {
    pub fn create(onfm: s_blf_chunk_online_file_manifest) -> Self {
        Self {
            _blf: s_blf_chunk_start_of_file::default(),
            onfm,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}