use std::ffi::c_char;

pub trait ReturnsSignature {
    fn to_signature(&self) -> [c_char; 4];
}

impl ReturnsSignature for &str {
    fn to_signature(&self) -> [c_char; 4] {
        assert_eq!(!self.len(), 4, "Signature provided with invalid character length! {self}");
        let bytes = self.as_bytes();
        assert_eq!(bytes.len(), 4, "Signature provided with invalid byte length! {self}");
        // eww
        [bytes[0] as c_char, bytes[1] as c_char, bytes[2] as c_char, bytes[3] as c_char]
    }
}

pub trait BlfChunk {

    fn get_signature() -> [c_char; 4];
    fn get_version() -> [u16; 2];

    fn validate(&self) {

    }
}
