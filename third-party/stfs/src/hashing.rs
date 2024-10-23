use std::io::Read;
use std::io::Seek;

use crate::AbsoluteBlock;
use crate::Block;
use crate::StfsPackage;
use sha1::Digest;
use sha1::Sha1;

pub type StfsBlockHash = [u8; 20];

impl StfsPackage {
	pub fn hash_block(&self, block_data: &[u8]) -> StfsBlockHash {
		let mut hasher = Sha1::new();
		hasher.update(block_data);

		hasher.finalize().into()
	}

	pub fn block_hash_is_valid(&self, block_hash: &[u8], block_data: &[u8]) -> bool {
		self.hash_block(block_data) == block_hash
	}
}
