use binrw::NullString;
use binrw::NullWideString;
use chrono::prelude::*;
use chrono::Duration;
use serde::Serializer;

use crate::StfTimestamp;

pub fn serialize_null_string<S>(x: &NullString, s: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	s.serialize_str(x.to_string().as_str())
}

pub fn serialize_null_wide_string<S>(x: &NullWideString, s: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	s.serialize_str(x.to_string().as_str())
}

pub fn stf_timestamp_to_chrono(timestamp: StfTimestamp) -> DateTime<Local> {
	Local
		.with_ymd_and_hms(
			1980 + (timestamp.year() as i32),
			timestamp.month() as u32,
			timestamp.day() as u32,
			timestamp.hour() as u32,
			timestamp.minute() as u32,
			(timestamp.seconds() as u32) << 1,
		)
		.unwrap()
}

pub fn windows_filetime_to_chrono(high: u32, low: u32) -> DateTime<Utc> {
	let time_as_i64 = (((high as u64) << 32) | low as u64) as i64;
	Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0).unwrap() + Duration::nanoseconds(time_as_i64 * 100)
}

#[cfg(test)]
mod tests {}
