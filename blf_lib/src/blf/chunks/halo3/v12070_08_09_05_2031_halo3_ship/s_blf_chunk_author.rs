use binrw::{binrw};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::{BlfChunkHooks, TitleAndBuild};
use blf_lib_derive::BlfChunk;
use crate::types::build_number_identifier::build_number_identifier;
use crate::types::c_string::StaticString;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("athr", 3.1)]
#[Size(0x44)]
#[brw(big)]
pub struct s_blf_chunk_author {
    pub program_name: StaticString<16>,
    pub build_identifier: build_number_identifier,
    pub build_string: StaticString<28>,
    pub author_name: StaticString<16>,
}

impl BlfChunkHooks for s_blf_chunk_author {}

impl s_blf_chunk_author {
    pub fn new(build_name: &str, build_identifier: build_number_identifier, build_string: &str, author_name: &str) -> s_blf_chunk_author {
        s_blf_chunk_author {
            program_name: StaticString::from_string(build_name.to_string()).unwrap(),
            build_identifier,
            build_string: StaticString::from_string(build_string.to_string()).unwrap(),
            author_name: StaticString::from_string(author_name.to_string()).unwrap(),
        }
    }

    pub fn for_build<T: TitleAndBuild>() -> s_blf_chunk_author {
        // untracked builds use -1.
        let mut build_number: u32 = 0xFFFFFFFF;
        let parsed_build_number = T::get_build_string()[..5].parse::<u32>();
        if parsed_build_number.is_ok() { build_number = parsed_build_number.unwrap(); }

        let version = env!("CARGO_PKG_VERSION");
        let name = env!("CARGO_PKG_NAME");

        Self {
            program_name: StaticString::from_string(format!("{name} v{version}")).unwrap(),
            build_identifier: build_number_identifier {
                build_number,
                build_number_version: 1,
            },
            build_string: StaticString::from_string(T::get_build_string().to_string()).unwrap(),
            author_name: Default::default(),
        }
    }
}
