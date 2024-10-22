// This module is based on ManagedDonkey's real_math module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/math/real_math.hpp

#![allow(dead_code)]

use std::io::Cursor;
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

pub fn dequantize_real(quantized: i32, min_value: f32, max_value: f32, size_in_bits: usize, exact_midpoint: bool) -> f32 {
    assert!(size_in_bits > 0, "size_in_bits>0");
    assert!(max_value > min_value, "max_value>min_value");
    assert!(!exact_midpoint || size_in_bits > 1, "!exact_midpoint || size_in_bits>1");

    let mut step_count = (1 << size_in_bits) - 1;
    if exact_midpoint {
        step_count -= step_count % 2;
    }
    assert!(step_count > 0, "step_count>0");

    let dequantized: f32;

    if quantized != 0 {
        if quantized < step_count {
            dequantized = (((step_count - quantized) as f32 * min_value) + (quantized as f32 * max_value)) / step_count as f32;
        }
        else {
            dequantized = max_value;
        }
    } else {
        dequantized = min_value;
    }

    if exact_midpoint && 2 * quantized == step_count {
        assert!(dequantized == (min_value + max_value) / 2.0, "value==(max_value+min_value)/2");
    }

    dequantized
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
    a1.atan2(a2)
}

pub fn dot_product3d(a1: &vector3d, a2: &vector3d) -> f32 {
    (a1.i * a2.i) + (a1.j * a2.j) + (a1.k * a2.k)
}

pub const k_test_real_epsilon: f32 = 0.001;
pub const k_real_epsilon: f32 = 0.0001;
pub const k_pi: f32 = 3.1415927;

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

pub fn quantize_normalized_vector3d(vector: &vector3d) -> i32 {
    assert!(assert_valid_real_normal3d(vector));

    let mut axis_code: u8;
    let u: f32;
    let v: f32;
    let negative: bool;
    let positive_code: u8;

    let i_abs = vector.i.abs();
    let j_abs = vector.j.abs();
    let k_abs = vector.k.abs();
    let i = vector.i;
    let j = vector.j;
    let k = vector.k;

    if i_abs <= j_abs && j_abs > k_abs {
        axis_code = 4;
        negative = j <= 0.0;
        positive_code = 1;
        u = i / j_abs;
        v = k / j_abs;
    } else if i_abs > j_abs && i_abs > k_abs {
        positive_code = 0;
        axis_code = 3;
        negative = i <= 0.0;
        u = j / i_abs;
        v = k / i_abs;
    } else {
        negative = k <= 0.0;
        positive_code = 2;
        axis_code = 5;
        v = j / k_abs;
        u = i / k_abs;
    }

    if !negative {
        axis_code = positive_code;
    }

    assert!(u >= -1.0 && u <= 1.0);
    assert!(v >= -1.0 && v <= 1.0);

    let quantized_u = quantize_real(u, -1.0, 1.0, 8, true, false);
    let quantized_v = quantize_real(v, -1.0, 1.0, 8, true, false);

    let result = axis_code as i32 | (quantized_u << 3) | (quantized_v << 11);

    result
}

pub fn square_root(value: f32) -> f32 {
    value.sqrt()
}

pub fn magnitude_squared3d(a1: &vector3d) -> f32 {
    (a1.i * a1.i) + (a1.j * a1.j) + (a1.k * a1.k)
}

fn magnitude3d(vector: &vector3d) -> f32 {
    square_root(magnitude_squared3d(vector))
}

fn scale_vector3d(vector: &mut vector3d, scale: f32) {
    vector.i *= scale;
    vector.j *= scale;
    vector.k *= scale;
}

pub fn normalize3d(vector: &mut vector3d) -> f32 {
    let mut result = magnitude3d(vector);

    if result.abs() >= k_real_epsilon {
        let scale = 1.0 / result;
        scale_vector3d(vector, scale);
    } else {
        result = 0.0;
    }

    result
}

pub fn dequantize_unit_vector3d(value: i32, vector: &mut vector3d) {
    let face = value & 7;
    let x = dequantize_real((value >> 3) as u8 as i32, -1.0, 1.0, 8, true);
    let y = dequantize_real((value >> 11) as u8 as i32, -1.0, 1.0, 8, true);

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
            panic!("dequantize_unit_vector3d: bad face value {face} when reading unit vector");
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