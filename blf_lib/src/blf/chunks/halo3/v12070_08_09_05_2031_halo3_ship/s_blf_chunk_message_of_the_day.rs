use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinReaderExt, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Signature("motd")]
#[Version(1.1)]
pub struct s_blf_chunk_message_of_the_day
{
    motd_length: u32,
    motd_message: String,
}

impl BlfChunkHooks for s_blf_chunk_message_of_the_day {}

impl BinWrite for s_blf_chunk_message_of_the_day {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        writer.write_be(&self.motd_length)?;
        writer.write_be(&self.motd_message.as_bytes())?;
        Ok(())
    }
}

impl BinRead for s_blf_chunk_message_of_the_day {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let length: u32 = reader.read_be()?;
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        let message = String::from_utf8(bytes).unwrap();

        Ok(Self {
            motd_length: length,
            motd_message: message,
        })
    }
}

impl s_blf_chunk_message_of_the_day {
    pub fn new(motd_message: String) -> s_blf_chunk_message_of_the_day {
        let mut motd = s_blf_chunk_message_of_the_day::default();
        motd.set_message(motd_message);
        motd
    }

    pub fn set_message(&mut self, motd_message: String) {
        self.motd_message = motd_message;
        self.motd_length = self.motd_message.len() as u32;
    }

    pub fn get_message(&self) -> &String {
        &self.motd_message
    }
}