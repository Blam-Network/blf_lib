use inline_colorization::{color_red, style_reset};
use blf_lib::blf::chunks::DynTitleAndBuild;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::v12070_08_09_05_2031_halo3_ship;

pub mod halo3;

pub const LANGUAGE_CODES: [&str; 10] = [
    "cht",
    "de",
    "en",
    "fr",
    "it",
    "jpn",
    "kor",
    "mx",
    "pt",
    "sp",
];

pub trait TitleConverter: DynTitleAndBuild {
    fn build_blfs(&mut self, config_path: &String, blfs_path: &String);
    fn build_config(&mut self, blfs_path: &String, config_path: &String);
}

#[macro_export]
macro_rules! title_converter {
    ($i:item) => {
        #[derive(blf_lib::derive::TitleAndBuild, Default)]
        $i
    }
}

fn get_title_converters() -> Vec<Box<dyn TitleConverter>> {
    vec![
        Box::new(v12070_08_09_05_2031_halo3_ship::default())
    ]
}

pub fn get_title_converter (title: String, build: String) -> Option<Box<dyn TitleConverter>> {
    for title_converter in get_title_converters() {
        if title_converter.title() == title && title_converter.build_string() == build {
            return Some(title_converter);
        }
    }

    None
}

pub fn fail_step(message: String) {
    println!("{color_red}failed{style_reset}.");
    panic!("{}", message);
}