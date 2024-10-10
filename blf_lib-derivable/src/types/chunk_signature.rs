use std::ffi::c_char;
use std::fmt::{Display, Formatter, Result};
use bincode::{Decode, Encode};

#[derive(Clone, Copy, Default, Debug, Encode, Decode, PartialEq)]
pub struct chunk_signature {
    value: [c_char; 4],
}

impl chunk_signature {
    pub fn new(value: [c_char; 4]) -> chunk_signature {
        chunk_signature {
            value,
        }
    }

    pub fn from_string(value: &str) -> chunk_signature {
        let mut array: [c_char; 4] = [0; 4];
        let bytes = value.as_bytes();
        
        if value.len() != 4 {
            panic!("Invalid chunk signature length");
        }

        if bytes.len() != 4 {
            panic!("Invalid chunk signature byte length");
        }

        for i in 0..4 {
            array[i] = bytes[i] as c_char;
        }

        chunk_signature {
            value: array,
        }
    }
}

impl Display for chunk_signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut string = String::new();
        for i in 0..4 {
            string.push(self.value[i] as u8 as char);
        }
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_signature() {
        let signature = chunk_signature::from_string("_blf");
        assert_eq!(signature.to_string(), "_blf");
    }
}