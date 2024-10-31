use std::io::{Cursor, Read};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use blf_lib::io::packed_decoding::{seek_pad, PackedDecoder};
use blf_lib::io::packed_encoding::PackedEncoder;
use blf_lib_derivable::io::endian::Endianness;
use blf_lib_derivable::io::packing::Packing;
use chrono::{NaiveDateTime, TimeZone, Utc};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct time_t(u64);

impl time_t {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn from_u64(t: u64) -> Self {
        Self {
            0: t,
        }
    }
}

impl Serialize for time_t {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let datetime = Utc.timestamp_opt(self.0 as i64, 0).single()
            .ok_or_else(|| serde::ser::Error::custom("Invalid timestamp"))?;
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for time_t {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(time_t(datetime.and_utc().timestamp() as u64))
    }
}
impl PackedEncoder for time_t {
    fn encode_packed(&self, endian: Endianness, packing: Packing) -> Vec<u8> {
        match endian {
            Endianness::Little => { packing.create_packed_buffer_from_slice(self.0.to_le_bytes().as_slice()) }
            Endianness::Big => { packing.create_packed_buffer_from_slice(self.0.to_be_bytes().as_slice()) }
        }
    }
}

impl PackedDecoder for time_t {
    fn decode_packed(reader: &mut Cursor<&[u8]>, endian: Endianness, packing: Packing) -> Result<Self, String>  {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes).map_err(|_|"Failed to read bytes.")?;
        seek_pad(reader, &bytes, packing)?;

        Ok(Self {0: match endian {
            Endianness::Little => { u64::from_le_bytes(bytes) }
            Endianness::Big => { u64::from_be_bytes(bytes) }
        }})
    }
}

