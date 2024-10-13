use std::ffi::{c_char};
use std::u32;
use blf_lib::blf_chunk;
use blf_lib::types::byte_limited_utf8_string::ByteLimitedUTF8String;
use blf_lib_derivable::blf::chunks::SerializableBlfChunk;

const MAX_BANHAMMER_MESSAGE_COUNT: usize = 32usize;
const BANHAMMER_MESSAGE_LENGTH: usize = 0x100;

blf_chunk!(
    #[Signature("bhms")]
    #[Version(1.1)]
    pub struct s_blf_chunk_banhammer_messages
    {
        message_count: u32,
        pub messages: Vec<ByteLimitedUTF8String<BANHAMMER_MESSAGE_LENGTH>> // UTF bytes,
    }
);

impl s_blf_chunk_banhammer_messages {
    pub fn get_messages(&self) -> Vec<String> {
        self.messages.iter().map(|message|message.get_string()).collect()
    }

    pub fn set_messages(&mut self, messages: Vec<String>) -> Result<(), String> {
        if messages.len() > MAX_BANHAMMER_MESSAGE_COUNT {
            return Err(format!("Too many banhammer messages! {}/{MAX_BANHAMMER_MESSAGE_COUNT}", messages.len()))
        }

        self.messages = Vec::with_capacity(messages.len());
        for message in messages.iter() {
            let message = ByteLimitedUTF8String::from_string(message);

            if !message.is_ok() {
                return Err(format!("Banhammer message: {}", message.unwrap_err()))
            }

            let message = message?;

            self.messages.push(message);
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
            let message: ByteLimitedUTF8String<BANHAMMER_MESSAGE_LENGTH>
                = bincode::decode_from_slice(&buffer[4 + (i * BANHAMMER_MESSAGE_LENGTH)..], config).unwrap().0;
            self.messages.push(message);
        }
    }
}