use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize, Default)]
#[brw(repr = u16)]
pub enum byte_order_mark {
    #[default]
    little_endian = 0xFFFE,
    big_endian = 0xFEFF
}