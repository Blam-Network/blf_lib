// This module is based on ManagedDonkey's integer_math module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/math/integer_math.hpp

#![allow(dead_code)]

#[derive(Clone, Copy, Default)]
pub struct int32_point3d  {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}