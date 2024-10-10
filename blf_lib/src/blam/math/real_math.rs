// This module is based on ManagedDonkey's real_math module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/math/real_math.hpp

#![allow(dead_code)]

use blf_lib_derive::TestSize;

const k_3d_count: usize = 3;

#[derive(Clone, Copy, TestSize)]
#[Size(0xC)]
#[PackedEncode(1)]
struct vector3d_coordinates {
    i: f32,
    j: f32,
    k: f32,
}

pub union vector3d  {

    coordinates: vector3d_coordinates,
    n: [f32; k_3d_count]
}
