pub struct Packing {
    packing: u8
}

pub const PACK1: Packing = Packing { packing: 1 };
pub const PACK2: Packing = Packing { packing: 2 };
pub const PACK4: Packing = Packing { packing: 4 };
pub const PACK8: Packing = Packing { packing: 8 };
pub const PACK16: Packing = Packing { packing: 16 };

const valid_packing_values: [u8; 5] = [
    1,
    2,
    4,
    8,
    16
];

impl Packing {
    pub fn new(packing: u8) -> Self {
        if !valid_packing_values.contains(&packing) {
            panic!("Invalid pack value: {packing}");
        }
        Packing { packing }
    }
}