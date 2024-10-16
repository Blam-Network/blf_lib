use serde::{Deserialize, Serialize};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_message_of_the_day_popup;

#[derive(Serialize, Deserialize)]
pub struct motd_popup {
    pub motdIdentifier: u32,
    pub acceptWaitMilliseconds: u32,
    pub title: String,
    pub heading: String,
    pub accept: String,
    pub wait: String,
    pub body: String,
}

impl motd_popup {
    pub fn to_chunk(self) -> Result<s_blf_chunk_message_of_the_day_popup, String> {
        s_blf_chunk_message_of_the_day_popup::create(
            self.motdIdentifier,
            self.acceptWaitMilliseconds,
            self.title,
            self.heading,
            self.accept,
            self.wait,
            self.body,
        )
    }

    pub fn from_chunk(chunk: s_blf_chunk_message_of_the_day_popup) -> motd_popup {
        motd_popup {
            motdIdentifier: chunk.title_index_identifier,
            acceptWaitMilliseconds: chunk.button_key_wait_time_ms,
            title: chunk.title.get_string(),
            heading: chunk.header.get_string(),
            accept: chunk.button_key.get_string(),
            wait: chunk.button_key_wait.get_string(),
            body: chunk.message.get_string(),
        }
    }
}
