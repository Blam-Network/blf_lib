use std::u32;
use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;

const k_banhammmer_messages_max_messages: usize = 32usize;
const k_banhammer_message_max_length: usize = 0x100;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("bhms", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_banhammer_messages
{
    #[bw(try_calc(u32::try_from(messages.len())))]
    message_count: u32,
    #[br(count = message_count)]
    pub messages: Vec<StaticString<k_banhammer_message_max_length>> // UTF bytes,
}

impl BlfChunkHooks for s_blf_chunk_banhammer_messages {}

impl s_blf_chunk_banhammer_messages {
    pub fn get_messages(&self) -> Vec<String> {
        self.messages.iter().map(|message|message.get_string()).collect()
    }

    fn set_messages(&mut self, messages: Vec<String>) -> Result<(), String> {
        if messages.len() > k_banhammmer_messages_max_messages {
            return Err(format!("Too many banhammer messages! {}/{k_banhammmer_messages_max_messages}", messages.len()))
        }

        self.messages = Vec::with_capacity(messages.len());
        for message in messages.iter() {
            let message = StaticString::<k_banhammer_message_max_length>::from_string(message);

            if !message.is_ok() {
                return Err(format!("Banhammer message: {}", message.unwrap_err()))
            }

            let message = message?;

            self.messages.push(message);
        }
        // self.message_count = self.messages.len() as u32;
        Ok(())
    }

    pub fn create(messages: Vec<String>) -> s_blf_chunk_banhammer_messages {
        let mut new = Self::default();
        new.set_messages(messages).unwrap();
        new
    }
}