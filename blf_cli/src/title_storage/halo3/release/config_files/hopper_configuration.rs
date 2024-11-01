use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::c_hopper_configuration;
use crate::io::ordered_map;

#[derive(Serialize, Deserialize)]
pub struct hopper_configuration {
    #[serde(serialize_with = "ordered_map")]
    pub descriptions: HashMap<String, String>,
    pub configuration: c_hopper_configuration,
}
