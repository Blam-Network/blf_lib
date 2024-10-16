use serde::{Deserialize, Serialize};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_message_of_the_day_popup;

#[derive(Serialize, Deserialize)]
pub struct motd_popup {
    pub title_index_identifier: u32,
    pub button_key_wait_time_ms: u32,
    pub title: String,
    pub header: String,
    pub button_key: String,
    pub button_key_wait: String,
    pub message: String,
}

impl motd_popup {
    pub fn to_chunk(self) -> Result<s_blf_chunk_message_of_the_day_popup, String> {
        s_blf_chunk_message_of_the_day_popup::create(
            self.title_index_identifier,
            self.button_key_wait_time_ms,
            self.title,
            self.header,
            self.button_key,
            self.button_key_wait,
            self.message,
        )
    }

    pub fn from_chunk(chunk: s_blf_chunk_message_of_the_day_popup) -> motd_popup {
        motd_popup {
            title_index_identifier: chunk.title_index_identifier,
            button_key_wait_time_ms: chunk.button_key_wait_time_ms,
            title: chunk.title.get_string(),
            header: chunk.header.get_string(),
            button_key: chunk.button_key.get_string(),
            button_key_wait: chunk.button_key_wait.get_string(),
            message: chunk.message.get_string(),
        }
    }
}
