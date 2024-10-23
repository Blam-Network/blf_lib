use binrw::NullString;
use binrw::NullWideString;
use serde::Serializer;

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
