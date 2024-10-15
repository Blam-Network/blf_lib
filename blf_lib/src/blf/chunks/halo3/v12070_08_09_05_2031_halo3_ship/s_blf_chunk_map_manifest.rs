use std::ffi::c_char;
use crate::blf_chunk;

blf_chunk!(
    #[Signature("mapm")]
    #[Version(1.1)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_map_manifest
    {
        map_count: u32,
        data: Vec<[u8; 0x100]>,
    }
);

impl s_blf_chunk_map_manifest {
    pub fn add_rsa_signature(&mut self, signature: &[u8]) -> Result<(), String> {
        if signature.len() != 0x100 {
            return Err(String::from("signature length must be 0x100"));
        }
        self.data.push(signature.try_into().unwrap());
        self.map_count = self.data.len() as u32;
        Ok(())
    }

    pub fn get_rsa_signatures(&self) -> &Vec<[u8; 0x100]> {
        &self.data
    }
}