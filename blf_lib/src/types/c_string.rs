use std::ffi::c_char;
use std::fmt::Write;

pub fn to_string(chars: &[c_char]) -> String {
    let mut res = String::new();
    for char in chars {
        let copy: u8 = char.clone() as u8;
        if copy == 0 {
            break;
        }
        res.write_char(char::from(copy)).unwrap();
    }
    res
}

pub fn from_string_with_length(string: String, length: usize) -> Vec<c_char> {
    let mut vec = from_string(string);

    vec.resize(length, 0);

    vec
}

pub fn from_string(string: String) -> Vec<c_char> {
    let mut vec = Vec::new();

    let bytes = string.as_bytes();

    if string.len() != bytes.len() {
        panic!("Invalid string.");
    }

    for i in 0..bytes.len() {
        vec.push(bytes[i] as c_char);
    }

    vec
}