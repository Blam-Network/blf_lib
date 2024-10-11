use std::f32::INFINITY;
use std::ffi::{c_char, CStr};
use std::u32;
use blf_lib::blf_chunk;
use blf_lib_derivable::blf::chunks::SerializableBlfChunk;
use crate::types::c_string::from_string;

const MAX_BANHAMMER_MESSAGE_COUNT: usize = 32 as usize; // TODO: Figure out what this is.
const BANHAMMER_MESSAGE_LENGTH: usize = 0x100;

blf_chunk!(
    #[Signature("bhms")]
    #[Version(1.1)]
    pub struct s_blf_chunk_banhammer_messages
    {
        message_count: u32,
        messages: Vec<[u8; BANHAMMER_MESSAGE_LENGTH]> // UTF bytes,
    }
);

impl s_blf_chunk_banhammer_messages {
    pub fn get_messages(&self) -> Vec<String> {
        let mut messages = Vec::<String>::with_capacity(self.message_count as usize);
        for message in self.messages.iter() {
            messages.push(CStr::from_bytes_until_nul(message).unwrap().to_str().unwrap().to_string());
        }
        messages
    }

    pub fn set_messages(&mut self, messages: Vec<String>) -> Result<(), String> {
        if messages.len() > MAX_BANHAMMER_MESSAGE_COUNT {
            return Err(format!("Too many banhammer messages! {}/{MAX_BANHAMMER_MESSAGE_COUNT}", messages.len()))
        }

        self.messages = Vec::with_capacity(messages.len() as usize);
        for message in messages.iter() {
            let mut message_vec = Vec::with_capacity(BANHAMMER_MESSAGE_LENGTH);
            let message_bytes = message.as_bytes();

            if message_bytes.len() > BANHAMMER_MESSAGE_LENGTH {
                return Err(format!("Banhammer message too long! {}/{MAX_BANHAMMER_MESSAGE_COUNT} bytes.\r\nBad Message: {message}", message_bytes.len()))
            }

            message_vec.copy_from_slice(message_bytes);
            self.messages.push(<[u8; BANHAMMER_MESSAGE_LENGTH]>::try_from(message_vec).unwrap());
        }
        self.message_count = self.messages.len() as u32;
        Ok(())
    }
}

impl SerializableBlfChunk for s_blf_chunk_banhammer_messages {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
        let mut encoded = Vec::<u8>::with_capacity((self.message_count as usize) * BANHAMMER_MESSAGE_LENGTH + 4);
        let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();

        encoded.append(&mut bincode::encode_to_vec(self.message_count, config).unwrap());
        for x in &self.messages {
            encoded.append(&mut bincode::encode_to_vec(x, bincode::config::standard()).unwrap());
        }

        encoded
    }

    fn decode_body(&mut self, buffer: &[u8]) {
        let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();

        self.message_count = bincode::decode_from_slice(buffer, config).unwrap().0;
        self.messages = Vec::with_capacity(self.message_count as usize);

        for i in 0..self.message_count as usize {
            let mut message = [0u8; BANHAMMER_MESSAGE_LENGTH];
            message = bincode::decode_from_slice(&buffer[4 + (i * BANHAMMER_MESSAGE_LENGTH)..], config).unwrap().0;
            self.messages.push(message);
        }
    }
}