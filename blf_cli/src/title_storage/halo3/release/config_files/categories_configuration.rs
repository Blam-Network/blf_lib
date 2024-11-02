use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_game_hopper_custom_category;
use crate::io::ordered_map;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct category_configuration_and_descriptions {
    pub configuration: s_game_hopper_custom_category,
    #[serde(serialize_with = "ordered_map")]
    pub descriptions: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct categories_configuration {
    pub categories: Vec<category_configuration_and_descriptions>,
}
