use std::u32;
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::math::integer_math::{int16_point2d, int16_rectangle2d};
use blf_lib::blam::common::math::real_math::{real_point3d, real_vector3d, real_plane3d, real_point2d, real_matrix4x3, real_vector2d, real_rectangle2d};
use blf_lib::types::bool::s_bool;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};
use std::io::{Write};
use num_derive::FromPrimitive;
use blf_lib::types::c_string::StaticWcharString;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("srid", 2.1)]
#[brw(big)]
#[Size(0x5C)]
pub struct s_blf_chunk_service_record
{
    pub player_name: StaticWcharString<16>, // Wide, 16 characters max
    pub appearance_flags: u8,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub tertiary_color: Color,
    pub is_elite: PlayerModel,
    pub foreground_emblem: u8,
    pub background_emblem: u8,
    pub emblem_flags: u8,
    #[brw(pad_before = 1)]
    pub emblem_primary_color: Color,
    pub emblem_secondary_color: Color,
    pub emblem_background_color: Color,
    #[brw(pad_before = 2)]
    pub spartan_helmet: SpartanHelmet,
    pub spartan_left_shoulder: SpartanShoulder,
    pub spartan_right_shoulder: SpartanShoulder,
    pub spartan_body: SpartanBody,
    pub elite_helmet: EliteArmour,
    pub elite_left_shoulder: EliteArmour,
    pub elite_right_shoulder: EliteArmour,
    pub elite_body: EliteArmour,
    pub service_tag: StaticWcharString<5>,
    pub campaign_progress: i32,
    pub highest_skill: i32,
    pub total_exp: i32,
    pub unknown_insignia: i32,
    pub rank: Rank,
    pub grade: Grade,
    pub unknown_insignia2: i32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, FromPrimitive)]
#[brw(big, repr = u8)]
pub enum PlayerModel {
    #[default]
    Spartan = 0,
    Elite = 1,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, FromPrimitive)]
#[brw(big, repr = u8)]
pub enum SpartanHelmet {
    #[default]
    Default = 0,
    Cobra,
    Intruder,
    Ninja,
    Regulator,
    Ryu,
    Marathon,
    Scout,
    Odst,
    MarkV,
    Rogue,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, FromPrimitive)]
#[brw(big, repr = u8)]
pub enum SpartanShoulder {
    #[default]
    Default = 0,
    Cobra,
    Intruder,
    Ninja,
    Regulator,
    Ryu,
    Marathon,
    Scout,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, FromPrimitive)]
#[brw(big, repr = u8)]
pub enum SpartanBody {
    #[default]
    Default = 0,
    Cobra,
    Intruder,
    Ninja,
    Ryu,
    Regulator,
    Scout,
    Katana,
    Bungie,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, FromPrimitive)]
#[brw(big, repr = u8)]
pub enum EliteArmour {
    #[default]
    Default = 0,
    Predator,
    Raptor,
    Blades,
    Scythe,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, FromPrimitive)]
#[brw(big, repr = u32)]
pub enum Rank {
    #[default]
    None = 0,
    Recruit,
    Apprentice,
    Private,
    Corporal,
    Sergeant,
    GunnerySergeant,
    Lieutenant,
    Captain,
    Major,
    Commander,
    Colonel,
    Brigadier,
    General,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, FromPrimitive)]
#[brw(big, repr = u32)]
pub enum Grade {
    #[default]
    Grade1 = 0,
    Grade2,
    Grade3,
    Grade4,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, FromPrimitive)]
#[brw(big, repr = u8)]
pub enum Color {
    #[default]
    Steel = 0,
    Silver,
    White,
    Red,
    Mauve,
    Salmon,
    Orange,
    Coral,
    Peach,
    Gold,
    Yellow,
    Pale,
    Sage,
    Green,
    Olive,
    Teal,
    Aqua,
    Cyan,
    Blue,
    Cobalt,
    Sapphire,
    Violet,
    Orchid,
    Lavender,
    Crimson,
    RubyWine,
    Pink,
    Brown,
    Ran,
    Khaki,
}

impl BlfChunkHooks for s_blf_chunk_service_record {}
