use std::any::Any;
use binrw::{BinRead, BinWrite, BinWriterExt};
use crate::blf::s_blf_header::s_blf_header;
use crate::types::chunk_signature::chunk_signature;
use crate::types::chunk_version::chunk_version;

pub trait BlfChunk {
    fn get_signature() -> chunk_signature;
    fn get_version() -> chunk_version;
}

pub trait DynamicBlfChunk {
    fn signature(&self) -> chunk_signature;
    fn version(&self) -> chunk_version;
}

pub trait BlfChunkHooks {
    fn before_write(&mut self, _previously_written: &Vec<u8>) {}
    fn after_read(&mut self) {}
}

pub trait SerializableBlfChunk: DynamicBlfChunk + Any + Send + Sync {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8>;
    fn decode_body(&mut self, buffer: &[u8]);

    fn write(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
        let mut encoded_chunk = self.encode_body(previously_written);
        let header = s_blf_header {
            signature: self.signature(),
            version: self.version(),
            chunk_size: (encoded_chunk.len() + s_blf_header::size()) as u32,
        };

        let mut encoded = Vec::with_capacity(s_blf_header::size() + encoded_chunk.len());
        encoded.append(&mut header.encode());
        encoded.append(&mut encoded_chunk);

        encoded
    }

    fn as_any(&self) -> &dyn Any;
}

impl<T: DynamicBlfChunk + BinRead + BinWrite + Clone + Any + BlfChunkHooks + Send + Sync> SerializableBlfChunk for T
    where for<'a> <T as BinWrite>::Args<'a>: Default, for<'a> <T as BinRead>::Args<'a>: Default {
    fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> where for<'a> <T as BinWrite>::Args<'a>: Default {
        self.before_write(previously_written);

        let mut writer = std::io::Cursor::new(Vec::<u8>::new());
        writer.write_ne(self).unwrap();
        writer.get_ref().clone()
    }

    fn decode_body(&mut self, buffer: &[u8]) where for<'b> <T as BinRead>::Args<'b>: Default {
        let mut reader = std::io::Cursor::new(buffer);

        self.clone_from(&T::read_ne(&mut reader).unwrap());

        self.after_read();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait ReadableBlfChunk: BlfChunk + Sized + SerializableBlfChunk + Default {
    fn read(buffer: Vec<u8>, header: Option<s_blf_header>) -> Self {
        let offset = if header.is_some() { 0 } else { s_blf_header::size() };
        let header = header.unwrap_or_else(|| s_blf_header::decode(&buffer.as_slice()) );
        let end = (header.chunk_size as usize - s_blf_header::size()) - offset;
        let mut chunk = Self::default();
        chunk.decode_body(&buffer[offset..end]);
        chunk
    }
}

impl<T: BlfChunk + Sized + SerializableBlfChunk + Default> ReadableBlfChunk for T {

}

pub trait TitleAndBuild {
    fn get_title() -> &'static str;

    fn get_build_string() -> &'static str;
}

pub trait DynTitleAndBuild {
    fn title(&self) -> String;

    fn build_string(&self) -> String;
}

pub trait ChunkFactory {
    fn decode(&self, signature: chunk_signature, version: chunk_version, buffer: Vec<u8>) -> Result<Box<dyn SerializableBlfChunk>, &str>;
}