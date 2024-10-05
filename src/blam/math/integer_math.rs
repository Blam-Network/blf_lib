// This module is based on ManagedDonkey's integer_math module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/math/integer_math.hpp

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[derive(Clone, Copy)]
struct int32_point3d_coordinates {
    x: u32,
    y: u32,
    z: u32,
}

pub union int32_point3d  {

    coordinates: int32_point3d_coordinates,
    n: [u32; 3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizeof_int32_point3d() {
        assert_eq!(size_of::<int32_point3d>(), 0xC);
    }
}
