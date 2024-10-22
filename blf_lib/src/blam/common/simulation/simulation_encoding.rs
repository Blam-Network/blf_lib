use blf_lib::blam::common::math::integer_math::int32_point3d;
use blf_lib::blam::common::math::real_math::{dequantize_real_point3d, real_point3d, real_rectangle3d};
use blf_lib::io::bitstream::c_bitstream_reader;
use crate::io::bitstream::c_bitstream_writer;
use crate::blam::common::math::real_math::quantize_real_point3d;

pub fn simulation_write_quantized_position(
    bitstream: &mut c_bitstream_writer,
    position: &real_point3d,
    bits: usize,
    a4: bool,
    world_bounds: &real_rectangle3d
) {

    let mut quantized_point = int32_point3d::default();
    quantize_real_point3d(position, world_bounds, bits, &mut quantized_point);

    if a4 {
        unimplemented!()
    }

    bitstream.write_point3d(&quantized_point, bits);
}

pub fn simulation_read_quantized_position(
    bitstream: &mut c_bitstream_reader,
    position: &mut real_point3d,
    axis_encoding_size_in_bits: usize,
    world_bounds: &real_rectangle3d
) {
    let mut point = int32_point3d::default();
    bitstream.read_point3d(&mut point, axis_encoding_size_in_bits);
    dequantize_real_point3d(&point, world_bounds, axis_encoding_size_in_bits, position);
}