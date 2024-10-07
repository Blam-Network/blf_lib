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