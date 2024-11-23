use std::u32;
use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticWcharString;
const k_motd_popup_title_max_length: usize = 48;
const k_motd_popup_header_max_length: usize = 48;
const k_motd_popup_button_key_max_length: usize = 48;
const k_motd_popup_button_key_wait_max_length: usize = 48;
const k_motd_popup_message_max_length: usize = 1024;



#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Signature("mtdp")]
#[Version(4.1)]
#[brw(big)]
pub struct s_blf_chunk_message_of_the_day_popup
{
    pub title_index_identifier: u32,
    pub button_key_wait_time_ms: u32,
    title_size: u32,
    pub title: StaticWcharString<k_motd_popup_title_max_length>,
    header_size: u32,
    pub header: StaticWcharString<k_motd_popup_header_max_length>,
    button_key_size: u32,
    pub button_key: StaticWcharString<k_motd_popup_button_key_max_length>,
    button_key_wait_size: u32,
    pub button_key_wait: StaticWcharString<k_motd_popup_button_key_max_length>,
    message_size: u32,
    pub message: StaticWcharString<k_motd_popup_message_max_length>,
}

impl BlfChunkHooks for s_blf_chunk_message_of_the_day_popup {}

impl s_blf_chunk_message_of_the_day_popup {
    pub fn create(
        title_index_identifier: u32,
        button_key_wait_time_ms: u32,
        title: String,
        header: String,
        button_key: String,
        button_key_wait: String,
        message: String,
    ) -> Result<Self, String> {
        Ok(s_blf_chunk_message_of_the_day_popup {
            title_index_identifier,
            button_key_wait_time_ms,
            title_size: (title.chars().count() * 2) as u32,
            header_size: (header.chars().count() * 2) as u32,
            button_key_size: (button_key.chars().count() * 2) as u32,
            button_key_wait_size: (button_key_wait.chars().count() * 2) as u32,
            message_size: (message.chars().count() * 2) as u32,
            title: StaticWcharString::from_string(&title)?,
            header: StaticWcharString::from_string(&header)?,
            button_key: StaticWcharString::from_string(&button_key)?,
            button_key_wait: StaticWcharString::from_string(&button_key_wait)?,
            message: StaticWcharString::from_string(&message)?
        })
    }
}