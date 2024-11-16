use std::error::Error;
use std::io::Read;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_map_manifest, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use crate::build_path;
use crate::io::get_files_in_folder;

pub const k_rsa_manifest_file_name: &str = "rsa_manifest.bin";
pub const k_rsa_signatures_config_folder_name: &str = "rsa_signatures";


blf_file! {
    pub struct rsa_manifest {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mapm: s_blf_chunk_map_manifest,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl rsa_manifest {
    pub fn create(mapm: s_blf_chunk_map_manifest) -> Self {
        Self {
            _blf: s_blf_chunk_start_of_file::default(),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            mapm: mapm.clone(),
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }

    pub fn build_for_hoppers(hoppers_config_path: &String) -> Result<rsa_manifest, Box<dyn Error>> {
        let rsa_folder = build_path!(hoppers_config_path, k_rsa_signatures_config_folder_name);
        let rsa_files = get_files_in_folder(&rsa_folder)?;

        if rsa_files.len() < 1 {
            return Err(Box::from("No RSA signatures found."))
        }

        let mut map_manifest = s_blf_chunk_map_manifest::default();

        for rsa_file_name in rsa_files {
            let rsa_file_path = build_path!(&rsa_folder, &rsa_file_name);
            let mut rsa_file = File::open(&rsa_file_path)?;
            let mut rsa_signature = Vec::<u8>::with_capacity(0x100);
            rsa_file.read_to_end(&mut rsa_signature).unwrap();

            let result = map_manifest.add_rsa_signature(rsa_signature.as_slice())?;
        }

        Ok(Self::create(map_manifest))
    }
}