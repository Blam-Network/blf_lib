// This module is based on ManagedDonkey's integer_math module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/math/integer_math.hpp

#![allow(dead_code)]

use blf_lib_derive::TestSize;

#[derive(Clone, Copy)]
struct int32_point3d_coordinates {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(TestSize)]
#[Size(0xC)]
pub union int32_point3d  {

    coordinates: int32_point3d_coordinates,
    n: [u32; 3]
}