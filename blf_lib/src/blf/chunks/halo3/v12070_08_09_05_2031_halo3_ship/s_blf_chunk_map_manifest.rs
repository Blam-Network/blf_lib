use crate::blf_chunk;
use crate::types::array::Array;

pub const k_map_manifest_max_signatures: usize = 128; // we're never hitting this...

blf_chunk!(
    #[Signature("mapm")]
    #[Version(1.1)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_map_manifest
    {
        map_count: u32,
        data: Vec<Array<u8, 0x100>>,
    }
);

impl s_blf_chunk_map_manifest {
    pub fn add_rsa_signature(&mut self, signature: &[u8]) -> Result<(), String> {
        if self.map_count >= k_map_manifest_max_signatures as u32 {
            return Err(format!("The map manifest is full! {} maps max", k_map_manifest_max_signatures));
        }

        if signature.len() != 0x100 {
            return Err(String::from("signature length must be 0x100"));
        }

        let arr = Array::from_slice(signature)?;

        self.data.push(arr);
        self.map_count = self.data.len() as u32;
        Ok(())
    }

    pub fn get_rsa_signatures(&self) -> &Vec<Array<u8, 0x100>> {
        &self.data
    }
}