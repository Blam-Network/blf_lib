use std::sync::Mutex;
use lazy_static::lazy_static;

pub fn crc_checksum_buffer (crc: u32, buffer: &Vec<u8>) -> u32 {
    crc32(crc, buffer)
}

lazy_static! {
    static ref CRC_TABLE: Mutex<[u32; 256]> = Mutex::new([0; 256]);
    static ref CRC_TABLE_INITIALIZED: Mutex<bool> = Mutex::new(false);
}

pub fn crc32(mut crc: u32, buffer: &Vec<u8>) -> u32 {
    let mut initialized = CRC_TABLE_INITIALIZED.lock().unwrap();
    if !*initialized {
        let mut table = CRC_TABLE.lock().unwrap();
        for byte in 0..256 {
            let mut crc = byte as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
            table[byte as usize] = crc;
        }
        *initialized = true;
    }
    drop(initialized);

    let table = CRC_TABLE.lock().unwrap();
    for &byte in buffer {
        crc = table[((crc ^ byte as u32) & 0xFF) as usize] ^ (crc >> 8);
    }
    crc
}