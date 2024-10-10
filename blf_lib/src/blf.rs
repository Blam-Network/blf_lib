use std::fs::File;
use std::io::Write;
use blf_lib_derivable::blf::chunks::SerializableBlfChunk;

pub mod chunks;
pub mod versions;
pub use blf_lib_derivable::blf::*;

pub mod derive {
    pub use blf_lib_derive;
}

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
}

impl BlfFile for BlfFileBuilder {
    fn write(&mut self, path: &str) {
        let mut data: Vec<u8> = Vec::new();

        for chunk in &mut self.chunks.iter_mut()  {
            data.append(&mut chunk.write(&data));
        }

        let file = File::create(path)
            .unwrap()
            .write_all(&data);
    }
}

#[macro_export]
macro_rules! blf_chunk {
    ($i:item) => {
        #[derive(blf_lib::derive::BlfChunk, Default, PartialEq, Debug, Clone)]
        $i
    }
}