use blf_lib_derivable::blf::chunks::{BlfChunk, Serializable};

pub mod halo3;

pub trait SerializableBlfChunk: BlfChunk + Serializable {}
