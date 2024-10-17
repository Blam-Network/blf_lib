// This module is based on ManagedDonkey's integer_math module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/math/integer_math.hpp

#![allow(dead_code)]

#[derive(Clone, Copy)]
pub struct int32_point3d_coordinates {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub union int32_point3d  {
    pub coordinates: int32_point3d_coordinates,
    pub n: [u32; 3]
}