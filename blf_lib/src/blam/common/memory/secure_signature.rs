use std::error::Error;
use binrw::{BinRead, BinWrite};
use serde::{Deserializer, Serialize, Serializer};

#[derive(Default, PartialEq, Debug, Clone, Copy, BinRead, BinWrite)]
pub struct s_network_http_request_hash {
    pub data: [u8; 20]
}

impl Serialize for s_network_http_request_hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&format!("{}", hex::encode_upper(self.data)))
    }
}

impl<'de> serde::Deserialize<'de> for s_network_http_request_hash {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(hex::decode(String::deserialize(d)?).unwrap().try_into().unwrap())
    }
}

impl TryFrom<Vec<u8>> for s_network_http_request_hash {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let data: [u8; 20];
        data = value.try_into().unwrap();

        Ok(s_network_http_request_hash {
            data
        })
    }
}