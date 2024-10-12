use std::ops::{Rem, Sub};

#[derive(Clone, Copy)]
pub struct Packing {
    packing: usize
}

pub const PACK1: Packing = Packing { packing: 1 };
pub const PACK2: Packing = Packing { packing: 2 };
pub const PACK4: Packing = Packing { packing: 4 };
pub const PACK8: Packing = Packing { packing: 8 };
pub const PACK16: Packing = Packing { packing: 16 };

const valid_packing_values: [usize; 5] = [
    1,
    2,
    4,
    8,
    16
];

impl Packing {
    pub fn new(packing: usize) -> Self {
        if !valid_packing_values.contains(&packing) {
            panic!("Invalid pack value: {packing}");
        }
        Packing { packing }
    }
}

impl Rem for Packing {
    type Output = ();

    fn rem(self, rhs: Self) -> Self::Output {
        self.packing % rhs.packing;
    }
}


impl Rem<Packing> for usize {
    type Output = ();

    fn rem(self, rhs: Packing) -> Self::Output {
        self % rhs.packing;
    }
}

impl Sub<()> for Packing {
    type Output = ();

    fn sub(self, rhs: ()) -> Self::Output {
        todo!()
    }
}