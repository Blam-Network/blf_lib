use thiserror::Error;

use crate::ConsoleKind;
use crate::RsaKeyKind;

#[derive(Debug, Error)]
pub enum Error {
	#[error("No private key exists for key kind {0:?} and console kind {1:?}")]
	NoPrivateKey(RsaKeyKind, ConsoleKind),
	#[error("RSA operation failed")]
	RsaError(#[from] rsa::Error),
}
