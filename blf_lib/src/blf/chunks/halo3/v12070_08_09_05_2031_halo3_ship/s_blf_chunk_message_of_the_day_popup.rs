use std::ffi::{c_char};
use std::u32;
use blf_lib::blf_chunk;
use blf_lib::types::byte_limited_wchar_string::ByteLimitedWcharString;

const MAX_BANHAMMER_MESSAGE_COUNT: usize = 32usize;
const k_motd_popup_title_max_length: usize = 48;
const k_motd_popup_header_max_length: usize = 48;
const k_motd_popup_button_key_max_length: usize = 48;
const k_motd_popup_button_key_wait_max_length: usize = 48;
const k_motd_popup_message_max_length: usize = 1024;



blf_chunk!(
    #[Signature("mtdp")]
    #[Version(4.1)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_message_of_the_day_popup
    {
        pub title_index_identifier: u32,
        pub button_key_wait_time_ms: u32,
        #[serde(skip_serializing)]
        title_size: u32,
        pub title: ByteLimitedWcharString<k_motd_popup_title_max_length>,
        #[serde(skip_serializing)]
        header_size: u32,
        pub header: ByteLimitedWcharString<k_motd_popup_header_max_length>,
        #[serde(skip_serializing)]
        button_key_size: u32,
        pub button_key: ByteLimitedWcharString<k_motd_popup_button_key_max_length>,
        #[serde(skip_serializing)]
        button_key_wait_size: u32,
        pub button_key_wait: ByteLimitedWcharString<k_motd_popup_button_key_max_length>,
        #[serde(skip_serializing)]
        message_size: u32,
        pub message: ByteLimitedWcharString<k_motd_popup_message_max_length>,
    }
);

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
        if title.len() > k_motd_popup_title_max_length {
            return Err(format!("Failed to create MOTD popup: Title is too long! {} / {k_motd_popup_title_max_length}", title.len()))
        }
        if header.len() > k_motd_popup_title_max_length {
            return Err(format!("Failed to create MOTD popup: Header is too long! {} / {k_motd_popup_header_max_length}", header.len()))
        }
        if button_key.len() > k_motd_popup_title_max_length {
            return Err(format!("Failed to create MOTD popup: Button Key is too long! {} / {k_motd_popup_button_key_max_length}", button_key.len()))
        }
        if button_key_wait.len() > k_motd_popup_button_key_wait_max_length {
            return Err(format!("Failed to create MOTD popup: Wait Button Key is too long! {} / {k_motd_popup_button_key_wait_max_length}", button_key_wait.len()))
        }
        if message.len() > k_motd_popup_button_key_wait_max_length {
            return Err(format!("Failed to create MOTD popup: Message is too long! {} / {k_motd_popup_message_max_length}", message.len()))
        }

        Ok(s_blf_chunk_message_of_the_day_popup {
            title_index_identifier,
            button_key_wait_time_ms,
            title_size: title.len() as u32,
            header_size: header.len() as u32,
            button_key_size: button_key.len() as u32,
            button_key_wait_size: button_key_wait.len() as u32,
            message_size: message.len() as u32,
            title: ByteLimitedWcharString::from_string(&title)?,
            header: ByteLimitedWcharString::from_string(&header)?,
            button_key: ByteLimitedWcharString::from_string(&button_key)?,
            button_key_wait: ByteLimitedWcharString::from_string(&button_key_wait)?,
            message: ByteLimitedWcharString::from_string(&message)?
        })
    }
}