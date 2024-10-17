#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

impl Endianness {
    pub fn new(big: bool) -> Self {
        if big {
            Self::Big
        }
        else {
            Self::Little
        }
    }
    pub fn swap(&self) -> Endianness {
        match self {
            Endianness::Little => Endianness::Big,
            Endianness::Big => Endianness::Little
        }
    }
}

pub fn get_platform_endianness() -> Endianness {
    if cfg!(target_endian = "little") {
        Endianness::Little
    }
    else if cfg!(target_endian = "big") {
        Endianness::Big
    }
    else {
        unreachable!()
    }
}