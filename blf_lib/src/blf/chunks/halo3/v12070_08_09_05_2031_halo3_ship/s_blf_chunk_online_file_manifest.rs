use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::types::byte_limited_utf8_string::StaticString;
use blf_lib_derive::PackedSerialize;
use crate::blf_chunk;

pub const k_file_manifest_max_files: usize = 128;

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, PackedSerialize)]
struct s_online_file {
    cache_key: StaticString<80>,
    hash: s_network_http_request_hash,
}

blf_chunk!(
    #[Signature("onfm")]
    #[Version(1.1)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_online_file_manifest
    {
        file_count: u32,
        data: Vec<s_online_file>,
    }
);

impl s_blf_chunk_online_file_manifest {
    pub fn add_file_hash(&mut self, cache_key: impl Into<String>, hash: s_network_http_request_hash) -> Result<(), String> {
        if self.file_count >= k_file_manifest_max_files as u32 {
            return Err(format!("The file manifest is full! {} files max", k_file_manifest_max_files));
        }

        self.data.push(s_online_file {
            cache_key: StaticString::from_string(cache_key)?,
            hash,
        });
        self.file_count = self.data.len() as u32;
        Ok(())
    }
}