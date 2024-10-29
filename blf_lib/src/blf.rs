use std::fs::File;
use std::io::{Read, Write};
use lazy_static::lazy_static;
use sha1::{Digest, Sha1};
use sha1::digest::Update;
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib_derivable::blf::chunks::{ChunkFactory, SerializableBlfChunk};

pub mod chunks;
pub mod versions;

pub use blf_lib_derivable::blf::s_blf_header::s_blf_header;

pub use blf_lib_derivable::blf::BlfFile;

pub struct BlfFileBuilder {
    chunks: Vec<Box<dyn SerializableBlfChunk>>,
}

impl BlfFileBuilder {
    pub fn new() -> BlfFileBuilder {
        BlfFileBuilder {
            chunks: Vec::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk: impl SerializableBlfChunk + 'static) -> &mut BlfFileBuilder {
        self.chunks.push(Box::new(chunk));
        self
    }

    fn add_dyn_chunk(&mut self, chunk: Box<dyn SerializableBlfChunk>) -> &mut BlfFileBuilder {
        self.chunks.push(chunk);
        self
    }

    pub fn get_chunks(&self) -> &Vec<Box<dyn SerializableBlfChunk>> {
        &self.chunks
    }
}

impl BlfFileBuilder {
    pub fn write(&mut self, path: &str) {
        let mut data: Vec<u8> = Vec::new();

        for chunk in &mut self.chunks.iter_mut()  {
            data.append(&mut chunk.write(&data));
        }

        let file = File::create(path)
            .unwrap()
            .write_all(&data);
    }

    pub fn read(path: &str, version: impl ChunkFactory) -> Self {
        let mut file = File::open(path).unwrap();
        let mut headerBytes = [0u8; s_blf_header::size()];
        let mut header: s_blf_header;
        let mut blf_file_builder = BlfFileBuilder::new();

        while file.read_exact(&mut headerBytes).is_ok() {
            header = s_blf_header::decode(&headerBytes);
            let body_bytes = vec![0u8; (header.chunk_size as usize) - s_blf_header::size()];
            blf_file_builder.add_dyn_chunk(version.decode(header.signature, header.version, body_bytes).unwrap());
        }

        blf_file_builder
    }
}

lazy_static!{
    static ref k_gen3_salt: Vec<u8> = hex::decode("EDD43009666D5C4A5C3657FAB40E022F535AC6C9EE471F01F1A44756B7714F1C36EC")
        .expect("Failed to generate hash salt!");
}

pub fn get_blf_file_hash(path: String) -> s_network_http_request_hash {
    let mut file = File::open(path).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    let mut hasher = Sha1::new();
    Update::update(&mut hasher, &k_gen3_salt);
    Update::update(&mut hasher, &bytes);
    s_network_http_request_hash::try_from(hasher.finalize().to_vec()).unwrap()
}

#[macro_export]
macro_rules! blf_chunk {
    ($i:item) => {
        #[derive(
            blf_lib::derive::BlfChunk,
            Default,
            PartialEq,
            Debug,
            Clone,
            serde::Serialize,
            serde::Deserialize,
        )]
        $i
    }
}

#[macro_export]
macro_rules! blf_file {
    ($i:item) => {
        #[derive(blf_lib::derive::BlfFile, Default, PartialEq, Debug, Clone)]
        $i
    }
}