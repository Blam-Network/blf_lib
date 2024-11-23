use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::array::StaticArray;

pub const k_map_manifest_max_signatures: usize = 128; // we're never hitting this...

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mapm", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_map_manifest
{
    #[bw(try_calc(u32::try_from(data.len())))]
    map_count: u32,
    #[br(count = map_count)]
    data: Vec<StaticArray<u8, 0x100>>,
}

impl BlfChunkHooks for s_blf_chunk_map_manifest {}

impl s_blf_chunk_map_manifest {
    pub fn add_rsa_signature(&mut self, signature: &[u8]) -> Result<(), String> {
        if self.data.len() >= k_map_manifest_max_signatures {
            return Err(format!("The map manifest is full! {} maps max", k_map_manifest_max_signatures));
        }

        if signature.len() != 0x100 {
            return Err(String::from("signature length must be 0x100"));
        }

        let arr = StaticArray::from_slice(signature)?;

        self.data.push(arr);
        // self.map_count = self.data.len() as u32;
        Ok(())
    }

    pub fn get_rsa_signatures(&self) -> &Vec<StaticArray<u8, 0x100>> {
        &self.data
    }
}