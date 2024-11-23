use std::ffi::c_char;
use binrw::{binrw};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::{BlfChunkHooks, TitleAndBuild};
use blf_lib_derive::BlfChunk;
use crate::types::build_number_identifier::build_number_identifier;
use crate::types::c_string::from_string_with_length;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Signature("athr")]
#[Version(3.1)]
#[Size(0x44)]
#[brw(big)]
pub struct s_blf_chunk_author {
    pub program_name: [c_char; 16],
    pub build_identifier: build_number_identifier,
    pub build_string: [c_char; 28],
    pub author_name: [c_char; 16],
}

impl BlfChunkHooks for s_blf_chunk_author {}

impl s_blf_chunk_author {
    pub fn new(build_name: &str, build_identifier: build_number_identifier, build_string: &str, author_name: &str) -> s_blf_chunk_author {
        s_blf_chunk_author {
            program_name: from_string_with_length(build_name.to_string(), 16).try_into().unwrap(),
            build_identifier,
            build_string: from_string_with_length(build_string.to_string(), 28).try_into().unwrap(),
            author_name: from_string_with_length(author_name.to_string(), 16).try_into().unwrap(),
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
            program_name: from_string_with_length(format!("{name} v{version}"), 16).try_into().unwrap(),
            build_identifier: build_number_identifier {
                build_number,
                build_number_version: 1,
            },
            build_string: from_string_with_length(T::get_build_string().to_string(), 28).try_into().unwrap(),
            author_name: Default::default(),
        }
    }
}
