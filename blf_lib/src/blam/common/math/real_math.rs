// This module is based on ManagedDonkey's real_math module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/math/real_math.hpp

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use blf_lib::blam::common::math::integer_math::int32_point3d;
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
    pub lower: f32,
    pub upper: f32,
}

#[derive(Default, PartialEq, Debug, Clone, PackedSerialize, Serialize, Deserialize)]
pub struct real_rectangle3d {
    pub x: real_bounds,
    pub y: real_bounds,
    pub z: real_bounds,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, PackedSerialize, Serialize, Deserialize)]
pub struct real_point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn quantize_real_point3d(
    point: &real_point3d,
    bounds: &real_rectangle3d,
    axis_encoding_bit_count: usize,
    quantized_point: &mut int32_point3d
) {
    assert!(axis_encoding_bit_count <= 32, "axis_encoding_bit_count<=SIZEOF_BITS(point->n[0])");

    let bounded_x =
        if point.x > bounds.x.upper { bounds.x.upper }
        else if point.x < bounds.x.lower { bounds.x.lower }
        else { point.x };
    let bounded_y =
        if point.y > bounds.y.upper { bounds.y.upper }
        else if point.y < bounds.y.lower { bounds.y.lower }
        else { point.y };
    let bounded_z =
        if point.y > bounds.z.upper { bounds.z.upper }
        else if point.z < bounds.z.lower { bounds.z.lower }
        else { point.z };

    quantized_point.x = quantize_real(bounded_x, bounds.x.lower, bounds.x.upper, axis_encoding_bit_count, false, false);
    quantized_point.y = quantize_real(bounded_x, bounds.y.lower, bounds.y.upper, axis_encoding_bit_count, false, false);
    quantized_point.z = quantize_real(bounded_x, bounds.z.lower, bounds.z.upper, axis_encoding_bit_count, false, false);
}

pub fn quantize_real(value: f32, min_value: f32, max_value: f32, size_in_bits: usize, exact_midpoint: bool, a6: bool) -> i32 {
    assert!(size_in_bits > 0, "size_in_bits>0");
    assert!(max_value > min_value, "max_value>min_value");
    assert!(!exact_midpoint || size_in_bits > 1, "!exact_midpoint || size_in_bits>1");
    assert!(value >= min_value, "value>=min_value");
    assert!(value <= max_value, "value<=max_value");

    let mut step_count = (1 << size_in_bits) - 1; // Maximum index based on size in bits
    if exact_midpoint {
        step_count -= step_count % 2; // Adjust for even distribution if exact midpoint is required
    }
    assert!(step_count > 0, "step_count>0");

    let step = (max_value - min_value) / step_count as f32;
    assert!(step > 0.0, "step>0.0f");

    let normalized_value = (value - min_value) / step;

    let sign = if normalized_value < 0.0 { -1.0 } else { 1.0 };
    let quantized_value = (sign * 0.5 + normalized_value) as i32;

    assert!(quantized_value >= 0 && quantized_value <= step_count, "quantized_value>=0 && quantized_value<=step_count");

    quantized_value
}