use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;

pub const k_file_manifest_max_files: usize = 128;

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
struct s_online_file {
    cache_key: StaticString<80>,
    hash: s_network_http_request_hash,
}

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Signature("onfm")]
#[Version(1.1)]
#[brw(big)]
pub struct s_blf_chunk_online_file_manifest
{
    #[bw(try_calc(u32::try_from(data.len())))]
    file_count: u32,
    #[br(count = file_count)]
    data: Vec<s_online_file>,
}

impl BlfChunkHooks for s_blf_chunk_online_file_manifest {}

impl s_blf_chunk_online_file_manifest {
    pub fn add_file_hash(&mut self, cache_key: impl Into<String>, hash: s_network_http_request_hash) -> Result<(), String> {
        if self.data.len() >= k_file_manifest_max_files {
            return Err(format!("The file manifest is full! {} files max", k_file_manifest_max_files));
        }

        self.data.push(s_online_file {
            cache_key: StaticString::from_string(cache_key)?,
            hash,
        });

        Ok(())
    }
}