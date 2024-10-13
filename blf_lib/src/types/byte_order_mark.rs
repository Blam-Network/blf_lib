use bincode::{Decode, Encode};
use blf_lib_derive::PackedSerialize;

#[derive(Clone, Copy, Encode, Decode, Debug, PartialEq, PackedSerialize)]
pub struct byte_order_mark {
    pub value: u16,
}

impl Default for byte_order_mark {
    fn default() -> byte_order_mark {
        little_endian
    }
}

pub const little_endian: byte_order_mark = byte_order_mark { value: u16::from_le_bytes([0xFF, 0xFE]) };
pub const big_endian: byte_order_mark = byte_order_mark { value: u16::from_le_bytes([0xFE, 0xFF]) };
