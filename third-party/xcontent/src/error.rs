use stfs::StfsError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum XContentError {
	#[error("Invalid XContent package magic")]
	InvalidMagic,
	#[error("Invalid XContent package header")]
	InvalidHeader,
	#[error("Invalid package type")]
	InvalidPackageType,
	#[error("Invalid STFS")]
	InvalidStfs(#[from] StfsError),
	#[error("I/O error")]
	Io(#[from] std::io::Error),
	#[error("I/O error (binrw)")]
	Binrw(#[from] binrw::Error),
}
