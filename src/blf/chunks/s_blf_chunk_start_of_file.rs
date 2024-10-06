use std::ffi::c_char;
use super::{BlfChunk, ReturnsSignature};

const k_tag_string_length: usize = 32;

#[repr(C, align(4))]
struct s_blf_chunk_start_of_file
{
    byte_order_mark: u16,

    name: [c_char; k_tag_string_length],
}

impl BlfChunk for s_blf_chunk_start_of_file {

    fn get_signature() -> [c_char; 4] {
        "_blf".to_signature()
    }

    fn get_version() -> [u16; 2] {
        [1, 2]
    }

    fn validate(&self) {
        if (self.byte_order_mark != 0xFEFF) && (self.byte_order_mark != 0xFFFE) {
            panic!("Invalid byte_order_mark on start of file!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sizeof_s_blf_chunk_start_of_file() {
        assert_eq!(size_of::<s_blf_chunk_start_of_file>(), 0x24);
    }
}
