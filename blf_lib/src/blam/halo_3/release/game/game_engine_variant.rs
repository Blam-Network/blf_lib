use std::fmt::{Debug};
use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinReaderExt, BinResult, BinWrite, BinWriterExt, Endian};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_default::c_game_engine_base_variant;
use blf_lib::blam::halo_3::release::game::game_engine_slayer::c_game_engine_slayer_variant;
use blf_lib::blam::halo_3::release::game::game_engine_assault::c_game_engine_assault_variant;
use blf_lib::blam::halo_3::release::game::game_engine_ctf::c_game_engine_ctf_variant;
use blf_lib::blam::halo_3::release::game::game_engine_infection::c_game_engine_infection_variant;
use blf_lib::blam::halo_3::release::game::game_engine_juggernaut::c_game_engine_juggernaut_variant;
use blf_lib::blam::halo_3::release::game::game_engine_king::c_game_engine_king_variant;
use blf_lib::blam::halo_3::release::game::game_engine_oddball::c_game_engine_oddball_variant;
use blf_lib::blam::halo_3::release::game::game_engine_sandbox::c_game_engine_sandbox_variant;
use blf_lib::blam::halo_3::release::game::game_engine_territories::c_game_engine_territories_variant;
use blf_lib::blam::halo_3::release::game::game_engine_vip::c_game_engine_vip_variant;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};

#[derive(BinRead, BinWrite, Serialize, Deserialize, Default, PartialEq, Debug, Copy, Clone, FromPrimitive)]
#[brw(repr = u32)]
pub enum e_game_engine {
    #[default]
    none = 0,
    ctf = 1,
    slayer = 2,
    oddball = 3,
    king = 4,
    sandbox = 5,
    vip = 6,
    juggernaut = 7,
    territories = 8,
    assault = 9,
    infection = 10,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_variant {
    pub m_game_engine: e_game_engine,
    pub m_base_variant: c_game_engine_base_variant,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_ctf_variant: Option<c_game_engine_ctf_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_slayer_variant: Option<c_game_engine_slayer_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_oddball_variant: Option<c_game_engine_oddball_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_king_variant: Option<c_game_engine_king_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_sandbox_variant: Option<c_game_engine_sandbox_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_vip_variant: Option<c_game_engine_vip_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_juggernaut_variant: Option<c_game_engine_juggernaut_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_territories_variant: Option<c_game_engine_territories_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_assault_variant: Option<c_game_engine_assault_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_infection_variant: Option<c_game_engine_infection_variant>,
}

impl BinWrite for c_game_variant {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        writer.write_ne(&self.m_game_engine)?;
        writer.write_ne(&self.m_base_variant)?;

        match self.m_game_engine {
            e_game_engine::none => { Ok(()) }
            e_game_engine::ctf => { writer.write_ne(&self.m_ctf_variant.as_ref().unwrap()) }
            e_game_engine::slayer => { writer.write_ne(&self.m_slayer_variant.as_ref().unwrap()) }
            e_game_engine::oddball => { writer.write_ne(&self.m_oddball_variant.as_ref().unwrap()) }
            e_game_engine::king => { writer.write_ne(&self.m_king_variant.as_ref().unwrap()) }
            e_game_engine::sandbox => { writer.write_ne(&self.m_sandbox_variant.as_ref().unwrap()) }
            e_game_engine::vip => { writer.write_ne(&self.m_vip_variant.as_ref().unwrap()) }
            e_game_engine::juggernaut => { writer.write_ne(&self.m_juggernaut_variant.as_ref().unwrap()) }
            e_game_engine::territories => { writer.write_ne(&self.m_territories_variant.as_ref().unwrap()) }
            e_game_engine::assault => { writer.write_ne(&self.m_assault_variant.as_ref().unwrap()) }
            e_game_engine::infection => { writer.write_ne(&self.m_infection_variant.as_ref().unwrap()) }
        }
    }
}

impl BinRead for c_game_variant {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let game_engine_index: e_game_engine = reader.read_ne()?;
        let base_game_engine: c_game_engine_base_variant = reader.read_ne()?;

        let mut game_variant = c_game_variant {
            m_game_engine: game_engine_index,
            m_base_variant: base_game_engine,
            m_ctf_variant: None,
            m_slayer_variant: None,
            m_oddball_variant: None,
            m_king_variant: None,
            m_sandbox_variant: None,
            m_vip_variant: None,
            m_juggernaut_variant: None,
            m_territories_variant: None,
            m_assault_variant: None,
            m_infection_variant: None,
        };

        match game_variant.m_game_engine {
            e_game_engine::none => {}
            e_game_engine::ctf => { game_variant.m_ctf_variant = reader.read_ne()?; }
            e_game_engine::slayer => { game_variant.m_slayer_variant = reader.read_ne()?; }
            e_game_engine::oddball => { game_variant.m_oddball_variant = reader.read_ne()?; }
            e_game_engine::king => { game_variant.m_king_variant = reader.read_ne()?; }
            e_game_engine::sandbox => { game_variant.m_sandbox_variant = reader.read_ne()?; }
            e_game_engine::vip => { game_variant.m_vip_variant = reader.read_ne()?; }
            e_game_engine::juggernaut => { game_variant.m_juggernaut_variant = reader.read_ne()?; }
            e_game_engine::territories => { game_variant.m_territories_variant = reader.read_ne()?; }
            e_game_engine::assault => { game_variant.m_assault_variant = reader.read_ne()?; }
            e_game_engine::infection => { game_variant.m_infection_variant = reader.read_ne()?; }
        }

        Ok(game_variant)
    }
}


impl c_game_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_raw(&self.m_game_engine, 4);

        self.m_base_variant.encode(bitstream);

        match self.m_game_engine {
            e_game_engine::none => { }
            e_game_engine::ctf => { self.m_ctf_variant.as_ref().unwrap().encode(bitstream); }
            e_game_engine::slayer => { self.m_slayer_variant.as_ref().unwrap().encode(bitstream); }
            e_game_engine::oddball => { self.m_oddball_variant.as_ref().unwrap().encode(bitstream); }
            e_game_engine::king => { self.m_king_variant.as_ref().unwrap().encode(bitstream); }
            e_game_engine::sandbox => { self.m_sandbox_variant.as_ref().unwrap().encode(bitstream); }
            e_game_engine::vip => { self.m_vip_variant.as_ref().unwrap().encode(bitstream); }
            e_game_engine::juggernaut => { self.m_juggernaut_variant.as_ref().unwrap().encode(bitstream); }
            e_game_engine::territories => { self.m_territories_variant.as_ref().unwrap().encode(bitstream); }
            e_game_engine::assault => { self.m_assault_variant.as_ref().unwrap().encode(bitstream); }
            e_game_engine::infection => { self.m_infection_variant.as_ref().unwrap().encode(bitstream); }
        }

    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        self.m_game_engine = bitstream.read_enum(4);
        self.m_base_variant.decode(bitstream);

        match self.m_game_engine {
            e_game_engine::none => { }
            e_game_engine::ctf => {
                self.m_ctf_variant = Some(c_game_engine_ctf_variant::default());
                self.m_ctf_variant.as_mut().unwrap().decode(bitstream);
            }
            e_game_engine::slayer => {
                self.m_slayer_variant = Some(c_game_engine_slayer_variant::default());
                self.m_slayer_variant.as_mut().unwrap().decode(bitstream);
            }
            e_game_engine::oddball => {
                self.m_oddball_variant = Some(c_game_engine_oddball_variant::default());
                self.m_oddball_variant.as_mut().unwrap().decode(bitstream);
            }
            e_game_engine::king => {
                self.m_king_variant = Some(c_game_engine_king_variant::default());
                self.m_king_variant.as_mut().unwrap().decode(bitstream);
            }
            e_game_engine::sandbox => {
                self.m_sandbox_variant = Some(c_game_engine_sandbox_variant::default());
                self.m_sandbox_variant.as_mut().unwrap().decode(bitstream);
            }
            e_game_engine::vip => {
                self.m_vip_variant = Some(c_game_engine_vip_variant::default());
                self.m_vip_variant.as_mut().unwrap().decode(bitstream);
            }
            e_game_engine::juggernaut => {
                self.m_juggernaut_variant = Some(c_game_engine_juggernaut_variant::default());
                self.m_juggernaut_variant.as_mut().unwrap().decode(bitstream);
            }
            e_game_engine::territories => {
                self.m_territories_variant = Some(c_game_engine_territories_variant::default());
                self.m_territories_variant.as_mut().unwrap().decode(bitstream);
            }
            e_game_engine::assault => {
                self.m_assault_variant = Some(c_game_engine_assault_variant::default());
                self.m_assault_variant.as_mut().unwrap().decode(bitstream);
            }
            e_game_engine::infection => {
                self.m_infection_variant = Some(c_game_engine_infection_variant::default());
                self.m_infection_variant.as_mut().unwrap().decode(bitstream);
            }
        }
    }
}