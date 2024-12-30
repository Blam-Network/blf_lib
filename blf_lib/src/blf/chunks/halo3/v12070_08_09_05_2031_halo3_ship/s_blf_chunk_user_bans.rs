use std::u32;
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::math::integer_math::{int16_point2d, int16_rectangle2d};
use blf_lib::blam::common::math::real_math::{real_point3d, real_vector3d, real_plane3d, real_point2d, real_matrix4x3, real_vector2d, real_rectangle2d};
use blf_lib::types::bool::s_bool;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};
use std::io::{Read, Seek, Write};
use blf_lib::types::c_string::StaticWcharString;
use blf_lib::types::time::time64_t;

pub const k_max_bans_count: usize = 32;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fubh", 2.1)]
#[brw(big)]
pub struct s_blf_chunk_user_bans
{
    #[bw(try_calc(u32::try_from(bans.len())))]
    ban_count: u32,
    pub unknown: u32,
    #[br(count = ban_count)]
    pub bans: Vec<s_blf_chunk_user_bans_ban>
}

#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
pub struct s_blf_chunk_user_bans_ban
{
    pub ban_type: BanType,
    pub ban_message_index: u32,
    pub start_time: Option<time64_t>,
    pub end_time: Option<time64_t>,
}

impl BinRead for s_blf_chunk_user_bans_ban {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let ban_type = BanType::read_options(reader, endian, ())?;
        let ban_message_index = u32::read_options(reader, endian, ())?;
        let start_time = match Option::<u64>::read_options(reader, endian, ())? {
            Some(val) => Some(time64_t(val)),
            None => None,
        };
        let end_time = match Option::<u64>::read_options(reader, endian, ())? {
            Some(val) => Some(time64_t(val)),
            None => None,
        };
        Ok(Self {
            ban_type,
            ban_message_index,
            start_time,
            end_time,
        })
    }
}

impl BinWrite for s_blf_chunk_user_bans_ban {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, _args: Self::Args<'_>) -> BinResult<()> {
        self.ban_type.write_options(writer, endian, ())?;
        self.ban_message_index.write_options(writer, endian, ())?;

        if let Some(start_time) = &self.start_time {
            Some(start_time.0).write_options(writer, endian, ())?;
        } else {
            None::<u64>.write_options(writer, endian, ())?;
        }

        if let Some(end_time) = &self.end_time {
            Some(end_time.0).write_options(writer, endian, ())?;
        } else {
            None::<u64>.write_options(writer, endian, ())?;
        }

        Ok(())
    }
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum BanType {
    #[default]
    Unknown1 = 0,
    Matchmaking = 1,
    Unknown2 = 2,
    Unknown3 = 3,
    Unknown4 = 4,
    Unknown5 = 5,
    Unknown6 = 6,
    Unknown7 = 7,
    XboxLIVE = 8,
    Unknown9 = 9
}

impl BlfChunkHooks for s_blf_chunk_user_bans {
    fn before_write(&mut self, _previously_written: &Vec<u8>) {
        if self.bans.len() > k_max_bans_count {
            println!("Tried to write a bans file with too many bans! ({}/{})", k_max_bans_count, self.bans.len());
            self.bans.resize(32, s_blf_chunk_user_bans_ban::default());
        }
    }
}
