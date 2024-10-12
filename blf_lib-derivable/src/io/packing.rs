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

    pub fn get_padding(&self, data_size: usize) -> usize {
        (self.packing - (data_size % self.packing)) % self.packing
    }

    pub fn create_buffer_for_type<T>(&self) -> Vec<u8> {
        Vec::<u8>::with_capacity(self.get_padding(size_of::<T>()))
    }

    pub fn create_packed_buffer_from_slice(&self, slice: &[u8]) -> Vec<u8> {
        let packed_length = slice.len() + self.get_padding(slice.len());
        let mut buffer = Vec::<u8>::with_capacity(packed_length);
        buffer.copy_from_slice(slice);
        buffer
    }
}