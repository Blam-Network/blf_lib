use blf_lib::io::endian::Endianness;
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib::io::packing::Packing;

impl PackedEncoder for u8 {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        packing.create_packed_buffer_from_slice(self.to_ne_bytes().as_slice())
    }
}

impl PackedEncoder for u16 {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        match endian {
            Endianness::Little => { packing.create_packed_buffer_from_slice(self.to_le_bytes().as_slice()) }
            Endianness::Big => { packing.create_packed_buffer_from_slice(self.to_be_bytes().as_slice()) }
        }
    }
}

impl PackedEncoder for u32 {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        match endian {
            Endianness::Little => { packing.create_packed_buffer_from_slice(self.to_le_bytes().as_slice()) }
            Endianness::Big => { packing.create_packed_buffer_from_slice(self.to_be_bytes().as_slice()) }
        }
    }
}

impl<const N: usize> PackedEncoder for [u8; N] {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        packing.create_packed_buffer_from_slice(self.as_ref())
    }
}

impl<const N: usize> PackedEncoder for [i8; N] {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        packing.create_packed_buffer_from_slice(
            &*self.iter().map(|byte| byte.to_ne_bytes()[0]).collect::<Vec<u8>>())
    }
}


impl PackedEncoder for [u8] {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        self.to_vec()
    }
}

impl PackedEncoder for String {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        packing.create_packed_buffer_from_slice(self.as_bytes())
    }
}

impl<T: PackedEncoder> PackedEncoder for Vec<T> {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        for element in self {
            buffer.append(&mut element.encode_packed(endian, packing));
        }
        buffer
    }
}