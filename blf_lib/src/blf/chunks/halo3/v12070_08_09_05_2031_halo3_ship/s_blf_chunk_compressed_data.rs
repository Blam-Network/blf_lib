use std::io::{Cursor, Read, Seek, Write};
use std::u32;
use binrw::{binrw, BinRead, BinReaderExt, BinResult, BinWrite, BinWriterExt, Endian};
use flate2::Compression;
use flate2::read::{ZlibDecoder};
use flate2::write::ZlibEncoder;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::{BlfChunk, BlfChunkHooks, ReadableBlfChunk, SerializableBlfChunk};
use blf_lib_derivable::blf::s_blf_header::s_blf_header;
use blf_lib_derivable::types::chunk_signature::chunk_signature;
use blf_lib_derivable::types::chunk_version::chunk_version;

lazy_static! {
    static ref signature: chunk_signature = chunk_signature::from_string("_cmp");
    static ref version: chunk_version = chunk_version::new(1.1);
}

// This BLF chunk doesn't use the typical BLFChunk derive because it takes a generic.
// So, we manually implement some of the traits that'd typically be automatically handled.
#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
pub struct s_blf_chunk_compressed_data<T> where T: BlfChunk + SerializableBlfChunk + Clone + ReadableBlfChunk
{
    // Not sure what the options for this are.
    pub compression_type: u8,
    pub chunk: T,
}

impl<T> blf_lib::blf::chunks::DynamicBlfChunk for s_blf_chunk_compressed_data<T> where T: BlfChunk + SerializableBlfChunk + Clone + ReadableBlfChunk {
    fn signature(&self) -> chunk_signature {
        *signature
    }

    fn version(&self) -> chunk_version {
        *version
    }
}
impl<T> BlfChunk for s_blf_chunk_compressed_data<T> where T: BlfChunk + SerializableBlfChunk + Clone + ReadableBlfChunk {
    fn get_signature() -> chunk_signature {
        *signature
    }

    fn get_version() -> chunk_version {
        *version
    }
}

impl<T> s_blf_chunk_compressed_data<T> where T: BlfChunk + SerializableBlfChunk + Clone + ReadableBlfChunk {
    pub fn create(chunk: T) -> Self {
        Self {
            chunk,
            compression_type: 0
        }
    }
}

impl<T> BinWrite for s_blf_chunk_compressed_data<T> where T: BlfChunk + SerializableBlfChunk + Clone + ReadableBlfChunk {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let data = self.chunk.clone().write(&Vec::new());
        let mut e = ZlibEncoder::new(Vec::new(), Compression::new(9));
        e.write_all(data.as_slice())?;
        let compressed_data = e.finish()?;

        writer.write_be(&self.compression_type)?;
        writer.write_be(&(data.len() as u32))?;
        writer.write_be(&compressed_data)?;

        Ok(())
    }
}

impl<T> BinRead for s_blf_chunk_compressed_data<T> where T: BlfChunk + SerializableBlfChunk + Clone + ReadableBlfChunk {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut compressed_bytes = Vec::new();

        // Read _cmp
        let compression_type: u8 = reader.read_be()?;
        let uncompressed_size = reader.read_be::<u32>()?; // uncompressed size
        reader.read_to_end(&mut compressed_bytes)?;

        // Uncompress _cmp
        let mut uncompressed_buffer = Vec::with_capacity(uncompressed_size as usize);
        let mut decoder = ZlibDecoder::new(Cursor::new(compressed_bytes));
        decoder.read_to_end(&mut uncompressed_buffer)?;

        // Read uncompressed chunk
        let (header_buffer, chunk_buffer) = uncompressed_buffer.split_at(s_blf_header::size());
        let header = s_blf_header::decode(&header_buffer);

        if header.signature != T::get_signature() || header.version != T::get_version() {
            return Err(binrw::error::Error::Custom {
                err: Box::new(format!("Unexpected compressed chunk, expected {} {} but got {} {}",
                        T::get_signature(), T::get_version(),
                        header.signature, header.version
                    )),
                pos: 5
            })
        }

        let chunk = T::read(chunk_buffer.into(), Some(header));

        Ok(Self {
            compression_type,
            chunk
        })
    }
}

impl<T> BlfChunkHooks for s_blf_chunk_compressed_data<T> where T: BlfChunk + SerializableBlfChunk + Clone + ReadableBlfChunk {}
