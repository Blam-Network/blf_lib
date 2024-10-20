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
    pub i: f32,
    pub j: f32,
    pub k: f32,
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
        if point.z > bounds.z.upper { bounds.z.upper }
        else if point.z < bounds.z.lower { bounds.z.lower }
        else { point.z };

    quantized_point.x = quantize_real(bounded_x, bounds.x.lower, bounds.x.upper, axis_encoding_bit_count, false, false);
    quantized_point.y = quantize_real(bounded_y, bounds.y.lower, bounds.y.upper, axis_encoding_bit_count, false, false);
    quantized_point.z = quantize_real(bounded_z, bounds.z.lower, bounds.z.upper, axis_encoding_bit_count, false, false);
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

pub fn dequantize_real(value: i32, min_value: f32, max_value: f32, size_in_bits: usize, exact_midpoint: bool, a6: bool) -> f32 {
    assert!(size_in_bits > 0, "size_in_bits>0");
    assert!(max_value > min_value, "max_value>min_value");
    assert!(!exact_midpoint || size_in_bits > 1, "!exact_midpoint || size_in_bits>1");

    let mut step_count = (1 << size_in_bits) - 1;
    if exact_midpoint {
        step_count -= step_count % 2;
    }
    assert!(step_count > 0, "step_count>0");

    let value_f32 = if value != 0 {
        if value < step_count {
            (((step_count - value) as f32 * min_value)
                + (value as f32 * max_value))
                / step_count as f32
        } else {
            max_value
        }
    } else {
        min_value
    };

    if exact_midpoint && 2 * value == step_count {
        assert!(value_f32 == (min_value + max_value) / 2.0, "value==(max_value+min_value)/2");
    }

    value_f32
}

pub fn assert_valid_real_normal3d(vector: &vector3d) -> bool {
    // Calculate the squared length of the vector and subtract 1.0
    let squared_length = vector.i * vector.i + vector.j * vector.j + vector.k * vector.k - 1.0;

    // Check if the result is not NaN or infinite
    if squared_length.is_nan() || squared_length.is_infinite() {
        return false;
    }

    // Check if the absolute value of the result is less than 0.001
    squared_length.abs() < 0.001
}

pub fn arctangent(a1: f32, a2: f32) -> f32 {
    let result = a1.atan2(a2);
    result
}

pub fn dot_product3d(a1: &vector3d, a2: &vector3d) -> f32 {
    (a1.i * a2.i) + (a1.j * a2.j) + (a1.k * a2.k)
}

pub const k_test_real_epsilon: f32 = 0.001;
pub const k_real_epsilon: f32 = 0.0001;

pub const global_up3d: vector3d = vector3d {
    i: 0f32,
    j: 0f32,
    k: 1f32,
};

pub const global_forward3d: vector3d = vector3d {
    i: 1f32,
    j: 0f32,
    k: 0f32,
};

pub const global_left3d: vector3d = vector3d {
    i: 0f32,
    j: 1f32,
    k: 0f32,
};

// Not sure exactly on this function name
pub fn quantize_unit_vector3d(vector: &vector3d) -> i32 {
    assert!(assert_valid_real_normal3d(vector));

    let mut largest_axis = 0;
    let mut axis_code: i32 = 3;
    let mut u = 0.0;
    let mut v = 0.0;

    let abs_x = vector.i.abs();
    let abs_y = vector.j.abs();
    let abs_z = vector.k.abs();

    if abs_x > abs_y && abs_x > abs_z {
        largest_axis = 0;
        axis_code = if vector.i > 0.0 { 3 } else { 0 };
        u = vector.j / abs_x;
        v = vector.k / abs_x;
    } else if abs_y > abs_z {
        largest_axis = 1;
        axis_code = if vector.j > 0.0 { 4 } else { 1 };
        u = vector.i / abs_y;
        v = vector.k / abs_y;
    } else {
        largest_axis = 2;
        axis_code = if vector.k > 0.0 { 5 } else { 2 };
        u = vector.i / abs_z;
        v = vector.j / abs_z;
    }

    assert!(u >= -1.0 && u <= 1.0);
    assert!(v >= -1.0 && v <= 1.0);

    let quantized_u = quantize_real(u, -1.0, 1.0, 8, true, false);
    let quantized_v = quantize_real(v, -1.0, 1.0, 8, true, false);

    let result = axis_code | (quantized_u << 3) | (quantized_v << 11);

    result
}

pub fn square_root(value: f32) -> f32 {
    value.sqrt()
}

pub fn magnitude_squared3d(a1: &vector3d) -> f32 {
    a1.i * a1.i + a1.j * a1.j + a1.k * a1.k
}

fn magnitude3d(vector: &vector3d) -> f32 {
    square_root(magnitude_squared3d(vector))
}

fn scale_vector3d(a1: &mut vector3d, scale: f32) {
    a1.i *= scale;
    a1.j *= scale;
    a1.k *= scale;
}

pub fn normalize3d(a1: &mut vector3d) -> f32 {
    let mut magnitude = magnitude3d(a1);

    if (magnitude - 0.0).abs() < k_real_epsilon {
        magnitude = 0.0;
    } else {
        let scale = 1.0 / magnitude;
        scale_vector3d(a1, scale);
    }

    magnitude
}

pub fn dequantize_unit_vector3d(value: i32, vector: &mut vector3d) {
    let face = value & 7;
    let x = dequantize_real(value >> 3, -1.0, 1.0, 8, true, false);
    let y = dequantize_real(value >> 11, -1.0, 1.0, 8, true, false);

    match face {
        0 => {
            vector.i = 1.0;
            vector.j = x;
            vector.k = y;
        }
        1 => {
            vector.i = x;
            vector.j = 1.0;
            vector.k = y;
        }
        2 => {
            vector.i = x;
            vector.j = y;
            vector.k = 1.0;
        }
        3 => {
            vector.i = -1.0;
            vector.j = x;
            vector.k = y;
        }
        4 => {
            vector.i = x;
            vector.j = -1.0;
            vector.k = y;
        }
        5 => {
            vector.i = x;
            vector.j = y;
            vector.k = -1.0;
        }
        _ => {
            vector.i = global_up3d.i;
            vector.i = global_up3d.j;
            vector.k = global_up3d.k;
        }
    }

    normalize3d(vector);
}

pub fn cross_product3d(a: &vector3d, b: &vector3d, out: &mut vector3d) {
    out.i = (a.j * b.k) - (a.k * b.j);
    out.j = (a.k * b.i) - (a.i * b.k);
    out.k = (a.i * b.j) - (a.j * b.i);
}

pub fn valid_real(value: f32) -> bool {
    !value.is_infinite() && !value.is_nan()
}

pub fn valid_realcmp(a1: f32, a2: f32) -> bool {
    valid_real(a1 - a2) && (a1 - a2).abs() < k_test_real_epsilon
}

pub fn valid_real_vector3d_axes3(forward: &vector3d, left: &vector3d, up: &vector3d) -> bool {
    assert_valid_real_normal3d(forward)
    && assert_valid_real_normal3d(left)
    && assert_valid_real_normal3d(up)
    && valid_realcmp(dot_product3d(forward, left), 0.0)
    && valid_realcmp(dot_product3d(left, up), 0.0)
    && valid_realcmp(dot_product3d(up, forward), 0.0)
}