use std::ffi::c_char;
use std::fmt::{Display, Formatter, Result};
use bincode::{Decode, Encode};

#[derive(Clone, Copy, Encode, Decode, Debug, PartialEq)]
pub struct build_number_identifier {
    pub build_number_version: u32,
    pub build_number: u32,
}

impl build_number_identifier {
    pub fn new(build_number_version: u32, build_number: u32) -> Self {
        build_number_identifier {
            build_number_version,
            build_number
        }
    }
}

impl Display for build_number_identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}.{}", self.build_number_version, self.build_number)
    }
}