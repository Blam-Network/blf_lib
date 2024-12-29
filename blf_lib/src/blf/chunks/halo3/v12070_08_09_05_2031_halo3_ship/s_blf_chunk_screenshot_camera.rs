use std::u32;
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::math::integer_math::{int16_point2d, int16_rectangle2d};
use blf_lib::blam::common::math::real_math::{real_point3d, real_vector3d, real_plane3d, real_point2d, real_matrix4x3, real_vector2d, real_rectangle2d};
use blf_lib::types::bool::s_bool;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("scnc", 2.1)]
#[brw(big)]
#[Size(0x164)]
pub struct s_blf_chunk_screenshot_camera
{
    pub jpeg_data_length: u32, // length of jpeg_data in the following scnd.
    pub camera: s_saved_camera,
    pub game_tick: u32,
    pub film_tick: u32,
}

#[derive(Default,PartialEq,Debug,Clone,BinRead, BinWrite,Serialize,Deserialize,TestSize)]
#[Size(0x158)]
pub struct s_saved_camera
{
    pub camera: render_camera,
    pub frustum_bounds: real_rectangle2d,
    pub projection: render_projection
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0xC0)]
pub struct render_projection {
    pub world_to_view: real_matrix4x3,
    pub view_to_world: real_matrix4x3,
    pub projection_bounds: real_rectangle2d,
    pub projection_matrix: [[f32; 4]; 4],
    pub world_to_screen_size: real_vector2d,
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x88)]
pub struct render_camera {
    pub position: real_point3d,
    pub forward: real_vector3d,
    pub up: real_vector3d,
    #[brw(pad_after = 3)]
    pub mirrored: s_bool,
    pub vertical_field_of_view: f32,
    pub field_of_view_scale: f32,
    pub window_pixel_bounds: int16_rectangle2d,
    pub window_title_safe_pixel_bounds: int16_rectangle2d,
    pub window_final_location: int16_point2d,
    pub render_pixel_bounds: int16_rectangle2d,
    pub render_title_safe_pixel_bounds: int16_rectangle2d,
    pub display_pixel_bounds: int16_rectangle2d,
    pub z_near: f32,
    pub z_far: f32,
    pub mirror_plane: real_plane3d,
    #[brw(pad_after = 3)]
    pub enlarge_view: s_bool,
    pub enlarge_center: real_point2d,
    pub enlarge_size_x: f32,
    pub enlarge_size_y: f32,
}


impl BlfChunkHooks for s_blf_chunk_screenshot_camera {}
