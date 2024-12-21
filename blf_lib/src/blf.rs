use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{Cursor, Read, Write};
use std::path::Path;
use lazy_static::lazy_static;
use sha1::{Digest, Sha1};
use sha1::digest::Update;
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib_derivable::blf::chunks::{BlfChunk, SerializableBlfChunk};

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

    pub fn get_chunk<T: BlfChunk + SerializableBlfChunk + 'static>(&self) -> Result<&T, Box<dyn Error>> {
        self.chunks
            .iter()
            .find_map(|chunk| chunk.as_ref().as_any().downcast_ref::<T>())
            .ok_or_else(|| format!("Chunk {} {} not found", T::get_signature(), T::get_version()).into())
    }

    pub fn write(&mut self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        for chunk in &mut self.chunks.iter_mut()  {
            data.append(&mut chunk.write(&data));
        }

        data
    }

    pub fn write_file(&mut self, path: impl Into<String>) {
        let path = &path.into();
        let data = self.write();
        
        let parent_path = Path::new(path).parent().unwrap();
        create_dir_all(parent_path).unwrap();

        let file = File::create(path)
            .unwrap()
            .write_all(&data);
    }

    pub fn read(&mut self, buffer: &Vec<u8>) -> Result<&mut BlfFileBuilder, Box<dyn Error>> {
        let mut reader = Cursor::new(buffer);
        let mut headerBytes = [0u8; s_blf_header::size()];
        let mut header: s_blf_header;

        for chunk in &mut self.chunks.iter_mut()  {
            reader.read_exact(&mut headerBytes)?;
            header = s_blf_header::decode(&headerBytes);

            if header.signature != chunk.signature() || header.version != chunk.version() {
                return Err(Box::from(format!("Failed to read chunk {} {}, found {} {} instead!",
                                             chunk.signature(),
                                             chunk.version(),
                                             header.signature,
                                             header.version
                )))
            }

            let mut body_bytes = vec![0u8; (header.chunk_size as usize) - s_blf_header::size()];
            reader.read_exact(&mut body_bytes)?;
            chunk.decode_body(&body_bytes);
        }

        Ok(self)
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }
}

// Used for everything we've encountered.
lazy_static!{
    static ref k_gen3_salt: Vec<u8> = hex::decode("EDD43009666D5C4A5C3657FAB40E022F535AC6C9EE471F01F1A44756B7714F1C36EC")
        .expect("Failed to generate hash salt!");
}

pub fn get_blf_file_hash(path: String) -> Result<s_network_http_request_hash, Box<dyn Error>> {
    let mut file = File::open(&path).map_err(|err| {
        Box::<dyn Error>::from(format!("get_blf_file_hash(\"{}\"): {}", path, err))
    })?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let mut hasher = Sha1::new();
    Update::update(&mut hasher, &k_gen3_salt);
    Update::update(&mut hasher, &bytes);
    let parsed = s_network_http_request_hash::try_from(hasher.finalize().to_vec());
    parsed
}

#[macro_export]
macro_rules! blf_file {
    ($i:item) => {
        #[derive(blf_lib::derive::BlfFile, Default, PartialEq, Debug, Clone, serde::Serialize)]
        $i
    }
}