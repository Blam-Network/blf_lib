use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use filesize::PathExt;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_message_of_the_day, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::io::read_file_to_string;
use blf_lib::types::byte_order_mark::byte_order_mark;
use crate::build_path;
use crate::io::create_parent_folders;
use crate::title_storage::check_file_exists;

pub const k_motd_file_name: &str = "motd.bin";
pub const k_mythic_motd_file_name: &str = "blue_motd.bin";
pub const k_motd_image_file_name: &str = "motd_image.jpg";
pub const k_mythic_motd_image_file_name: &str = "blue_motd_image.jpg";
pub const k_motd_config_folder: &str = "motd";
pub const k_mythic_motd_config_folder: &str = "motd_mythic";

const k_max_motd_image_size: u32 = 61440;
const k_motd_image_width: u16 = 476;
const k_motd_image_height: u16 = 190;

blf_file! {
    pub struct motd {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        motd: s_blf_chunk_message_of_the_day,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl motd {
    pub fn create(motd: String) -> Self {
        Self {
            _blf: s_blf_chunk_start_of_file::new("halo3 motd", byte_order_mark::little_endian),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            motd: s_blf_chunk_message_of_the_day::new(motd),
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }

    pub fn read_from_config(
        hoppers_config_path: &String,
        language_code: &str,
        mythic: bool,
    ) -> Result<motd, Box<dyn Error>> {
        Ok(motd::create(read_file_to_string(build_path!(
            hoppers_config_path,
            if mythic { k_mythic_motd_config_folder } else { k_motd_config_folder },
            format!("{language_code}.txt")
        ))?))
    }

    pub fn write_to_config(&self, hoppers_config_path: &String, language_code: &str, mythic: bool) -> Result<(), Box<dyn Error>> {
        let config_file_path = build_path!(
            hoppers_config_path,
            if mythic { k_mythic_motd_config_folder } else { k_motd_config_folder },
            format!("{language_code}.txt")
        );

        let messages_text = self.motd.get_message();

        create_parent_folders(&config_file_path)?;

        let mut text_file = File::create(config_file_path).unwrap();

        text_file.write_all(messages_text.as_bytes())?;

        Ok(())
    }

    pub fn validate_image(path: &String) -> Result<(), Box<dyn Error>> {
        if !check_file_exists(&path) {
            return Err(Box::from("No image file was found"));
        }

        let path = Path::new(&path);
        let image_filesize = path.size_on_disk()?;
        if image_filesize > k_max_motd_image_size as u64 {
            return Err(Box::from(format!("Image file size is too large ({}B > {}B)", image_filesize, k_max_motd_image_size)));
        }

        let jpeg_data = fs::read(path)?;
        let mut decoder = jpeg_decoder::Decoder::new(jpeg_data.as_slice());
        decoder.read_info()?;
        let header = decoder.info().unwrap();
            if header.width != k_motd_image_width {
            return Err(Box::from(format!("Invalid image width ({}px != {}px)", header.width, k_motd_image_width)));
        }
        if header.height != k_motd_image_height {
            return Err(Box::from(format!("Invalid image width ({}px != {}px)", header.height, k_motd_image_height)));
        }

        Ok(())
    }
}