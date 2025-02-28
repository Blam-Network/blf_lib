use std::fmt::Display;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

#[derive(Default, Clone, Debug, PartialEq, BinRead, BinWrite, Copy)]
pub struct time64_t(pub u64);

impl Display for time64_t {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let datetime = Utc.timestamp_opt(self.0 as i64, 0).single();
        if datetime.is_none() {
            return write!(f, "<Invalid Timestamp>");
        }
        let datetime = datetime.unwrap();
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        write!(f, "{formatted}")
    }
}

impl Serialize for time64_t {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let datetime = Utc.timestamp_opt(self.0 as i64, 0).single()
            .ok_or_else(|| serde::ser::Error::custom("Invalid timestamp"))?;
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for time64_t {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(time64_t(datetime.and_utc().timestamp() as u64))
    }
}

impl Into<u64> for time64_t {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for time64_t {
    fn from(t: u64) -> Self {
        Self { 0: t }
    }
}

impl Into<DateTime<Utc>> for time64_t {
    fn into(self) -> DateTime<Utc> {
        Utc.timestamp(self.0 as i64, 0)
    }
}

#[derive(Default, Clone, Debug, PartialEq, BinRead, BinWrite, Copy)]
pub struct time32_t(pub u32);

impl Display for time32_t {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let datetime = Utc.timestamp_opt(self.0 as i64, 0).single();
        if datetime.is_none() {
            return write!(f, "<Invalid Timestamp>");
        }
        let datetime = datetime.unwrap();
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        write!(f, "{formatted}")
    }
}

impl Serialize for time32_t {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let datetime = Utc.timestamp_opt(self.0 as i64, 0).single()
            .ok_or_else(|| serde::ser::Error::custom("Invalid timestamp"))?;
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for time32_t {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(time32_t(datetime.and_utc().timestamp() as u32))
    }
}

impl Into<u32> for time32_t {
    fn into(self) -> u32 {
        self.0
    }
}

impl Into<DateTime<Utc>> for time32_t {
    fn into(self) -> DateTime<Utc> {
        Utc.timestamp(self.0 as i64, 0)
    }
}

impl From<u32> for time32_t {
    fn from(t: u32) -> Self {
        Self { 0: t }
    }
}

#[derive(Default, Clone, Debug, PartialEq, BinRead, BinWrite)]
pub struct filetime(u64);

impl filetime {
    // FILETIME to UNIX epoch conversion constant: difference in seconds between 1601 and 1970
    const FILETIME_EPOCH_OFFSET: u64 = 11644473600;

    // Converts FILETIME to time_t (in seconds since 1970)
    pub fn to_time_t(&self) -> u64 {
        if self.0 == 0 { return 0 }
        (self.0 / 10_000_000) - Self::FILETIME_EPOCH_OFFSET
    }

    // Converts time_t (in seconds since 1970) to FILETIME (in 100-nanosecond intervals since 1601)
    pub fn from_time_t(t: u64) -> Self {
        Self((t + Self::FILETIME_EPOCH_OFFSET) * 10_000_000)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn from_u64(t: u64) -> Self {
        Self(t)
    }
}

impl Serialize for filetime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.0 == 0 {
            return serializer.serialize_str("None")
        }

        let seconds_since_unix_epoch = self.to_time_t();
        let datetime = Utc.timestamp_opt(seconds_since_unix_epoch as i64, 0)
            .single()
            .ok_or_else(|| serde::ser::Error::custom("Invalid timestamp"))?;
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for filetime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        if s == "None" {
            return Ok(Self(0))
        }

        let datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(Self::from_time_t(datetime.and_utc().timestamp() as u64))
    }
}
