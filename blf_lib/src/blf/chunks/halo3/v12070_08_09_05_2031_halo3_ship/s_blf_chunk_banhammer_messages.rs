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
    #[PackedSerialize(1, BigEndian)]
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