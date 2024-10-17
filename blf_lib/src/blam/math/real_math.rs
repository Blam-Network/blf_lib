// This module is based on ManagedDonkey's real_math module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/math/real_math.hpp

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use blf_lib_derive::PackedSerialize;

const k_3d_count: usize = 3;

#[derive(Default, PartialEq, Debug, Clone, Copy, PackedSerialize, Serialize, Deserialize)]
pub struct vector3d {
    i: f32,
    j: f32,
    k: f32,
}


#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct real_bounds {
    lower: f32,
    upper: f32,
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct real_rectangle3d {
    x: real_bounds,
    y: real_bounds,
    z: real_bounds,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, PackedSerialize, Serialize, Deserialize)]
pub struct real_point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}