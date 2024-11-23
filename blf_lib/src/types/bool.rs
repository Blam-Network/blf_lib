use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinReaderExt, BinResult, BinWrite, Endian};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct s_bool(pub bool);

impl Into<bool> for s_bool {
    fn into(self) -> bool {
        self.0
    }
}

impl From<bool> for s_bool {
    fn from(val: bool) -> s_bool {
        s_bool {
            0: val
        }
    }
}

impl PartialEq<bool> for s_bool {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

impl Default for s_bool {
    fn default() -> Self {
        s_bool(false) // Default value is false
    }
}

// Custom Serialize implementation
impl Serialize for s_bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the value directly as a boolean, not as an object
        self.0.serialize(serializer)
    }
}

// Custom Deserialize implementation
impl<'de> Deserialize<'de> for s_bool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize a boolean directly into the s_bool struct
        let value = bool::deserialize(deserializer)?;
        Ok(s_bool(value))
    }
}

impl BinRead for s_bool {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, binrw::Error> {
        // Standard read function for reading a single byte (boolean)
        let byte: u8 = reader.read_type(Endian::NATIVE)?;
        let value = byte != 0; // Interpreting 0 as false, any non-zero value as true
        Ok(s_bool(value))
    }

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        // Since we are only reading a single byte, endian doesn't affect this for a bool,
        // but this prepares it for future types where endian would matter.
        let byte: u8 = reader.read_type(endian)?;
        let value = byte != 0;

        Ok(s_bool(value))
    }
}

impl BinWrite for s_bool {
    type Args<'a> = ();

    fn write<W: Write>(&self, writer: &mut W) -> Result<(), binrw::Error> {
        let byte = if self.0 { 1u8 } else { 0u8 };
        writer.write(&byte.to_ne_bytes())?;
        Ok(())
    }

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, _args: Self::Args<'_>) -> BinResult<()> {
        // Since we are only writing a single byte, endian doesn't affect this for a bool,
        // but this prepares it for future types where endian would matter.
        let byte = if self.0 { 1u8 } else { 0u8 };

        match endian {
            Endian::Big => {
                writer.write(&byte.to_be_bytes())?;
            }
            Endian::Little => {
                writer.write(&byte.to_le_bytes())?;
            }
        }

        Ok(())
    }
}