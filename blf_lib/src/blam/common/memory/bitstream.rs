// This module is based on ManagedDonkey's bitstream module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/memory/bitstream.cpp

// Changes:
// - We don't use pointer arithmetic internally, as such m_data_max is removed.
// - The empty constructor is removed, function overloads are unsupported in Rust.

use std::cmp::min;
use std::mem;
use libc::wchar_t;
use widestring::U16CString;
use blf_lib::blam::common::math::real_math::{assert_valid_real_normal3d, cross_product3d, dequantize_unit_vector3d, dot_product3d, k_real_epsilon, global_forward3d, global_left3d, global_up3d, normalize3d, quantize_unit_vector3d, valid_real_vector3d_axes3, arctangent};
use blf_lib::io::packed_encoding::PackedEncoder;
use crate::blam::common::math::integer_math::int32_point3d;
use crate::blam::common::math::real_math::{k_test_real_epsilon, quantize_real, vector3d};
use crate::blam::common::networking::transport::transport_security::s_transport_secure_address;

#[derive(Default, PartialEq, Eq, Debug)]
pub enum e_bitstream_byte_order
{
    #[default]
    _bitstream_byte_order_little_endian,
    _bitstream_byte_order_big_endian
}

#[derive(Default, PartialEq, Eq, Debug)]
enum e_bitstream_state
{
    #[default]
    _bitstream_state_initial = 0,
    _bitstream_state_writing,
    _bitstream_state_write_finished,
    _bitstream_state_reading,
    _bitstream_state_read_only_for_consistency,
    _bitstream_state_read_finished,

    k_bitstream_state_count
}

const k_bitstream_maximum_position_stack_size: usize = 4;

#[derive(Default)]
struct s_bitstream_data {
    current_memory_bit_position: usize,
    current_stream_bit_position: usize,
    window: u64,
    window_bits_used: usize,
    current_stream_byte_position: usize, // aka current_memory_byte_position
}

pub struct c_bitstream<'a>
{
    m_data: &'a mut [u8],
    // m_data_max: u32, REMOVED
    m_data_size_bytes: usize,
    m_data_size_alignment: u32, // not sure if this is used
    m_state: e_bitstream_state,
    __unknown14: u32, // might be debug mode
    m_bitstream_data: s_bitstream_data,
    m_position_stack_depth: u32,
    __unknown34: u32,
    m_position_stack: [s_bitstream_data; k_bitstream_maximum_position_stack_size],
    __unknown98: u32,
    __unknown9C: u32,

    m_byte_order: e_bitstream_byte_order // new

}

impl<'a> c_bitstream<'a> {

    // pub fn new() {}

    pub fn new(data: &mut [u8], byte_order: e_bitstream_byte_order) -> c_bitstream {
        let length = data.len();
        c_bitstream {
            m_data: data,
            m_data_size_bytes: length,
            m_data_size_alignment: 1,
            m_state: e_bitstream_state::_bitstream_state_initial,
            __unknown14: Default::default(),
            m_bitstream_data: Default::default(),
            m_position_stack_depth: 0,
            __unknown34: Default::default(),
            m_position_stack: Default::default(),
            __unknown98: Default::default(),
            __unknown9C: Default::default(),
            m_byte_order: byte_order,
        }
    }

    // READS

    pub fn read_raw_data(value: &mut [u8], size_in_bits: u8) {
        unimplemented!()
    }

    pub fn read_signed_integer(size_in_bits: u8) -> u32 {
        unimplemented!()
    }

    pub fn read_qword(size_in_bits: u8) -> u64 {
        unimplemented!()
    }

    pub fn read_bool() -> bool {
        unimplemented!()
    }

    pub fn read_bits_internal(&mut self, output: &mut [u8], size_in_bits: usize) {
        let end_memory_position = output.len();
        let end_stream_position = self.m_data.len();
        let remaining_stream_bytes = end_stream_position - (self.m_bitstream_data.current_stream_byte_position + 1);
        let remaining_stream_bits = (8 - self.m_bitstream_data.current_stream_bit_position) + (remaining_stream_bytes * 8);

        let size_in_bytes = size_in_bits.div_ceil(8);
        if end_memory_position < size_in_bytes {
            panic!("Tried to read {size_in_bytes} bytes ({size_in_bits} bits) into a {end_memory_position} byte buffer!")
        }

        if remaining_stream_bits < size_in_bits {
            panic!("Tried to read {size_in_bits} bits but the stream only has {remaining_stream_bits} bits left!")
        }

        if size_in_bits == 0 {
            panic!("Tried to read zero bits.")
        }

        let mut windows_to_read = (size_in_bits as f32 / 64f32).ceil() as usize;
        let mut remaining_bits_to_read = size_in_bits;
        while windows_to_read > 0 {
            let mut window_bits_to_read = min(remaining_bits_to_read, 64);
            self.m_bitstream_data.window = 0;
            self.m_bitstream_data.window_bits_used = 0;

            // 1. Read any remaining bits on the current byte.
            let remaining_bits_at_position = 8 - self.m_bitstream_data.current_stream_bit_position;
            if remaining_bits_at_position < 8 {
                let current_byte_index = self.m_bitstream_data.current_stream_byte_position;

                let mut bits = self.m_data[current_byte_index];
                // Of the remaining bits at the current byte, how many are we reading?
                let reading_bits_at_position = min(size_in_bits, remaining_bits_at_position);

                // shift the byte to remove previously read bits.
                bits <<= self.m_bitstream_data.current_stream_bit_position;

                if reading_bits_at_position < remaining_bits_at_position {
                    // Mask off the excess
                    let mask = 0xff << (8 - size_in_bits);
                    bits &= mask;
                }

                // push the byte onto the window.
                self.m_bitstream_data.window = u64::from(bits) << (64 - 8);
                self.m_bitstream_data.window_bits_used = reading_bits_at_position;
                // If we read all the remaining bits at this byte, move to the next.
                if reading_bits_at_position == remaining_bits_at_position {
                    self.m_bitstream_data.current_stream_bit_position = 0;
                    self.m_bitstream_data.current_stream_byte_position += 1;
                } else {
                    self.m_bitstream_data.current_stream_bit_position += reading_bits_at_position;
                }
                window_bits_to_read -= reading_bits_at_position;
            }

            // 2. Read any full bytes.
            if window_bits_to_read >= 8 {
                let current_byte_index = self.m_bitstream_data.current_stream_byte_position;

                let bytes_to_read = window_bits_to_read / 8;
                let window_shift = 64 - (bytes_to_read * 8) - self.m_bitstream_data.window_bits_used;
                let mut window_bytes = [0u8; 8];
                let unused_bytes = 8 - bytes_to_read;
                window_bytes[unused_bytes..8].copy_from_slice(&self.m_data[current_byte_index..current_byte_index + bytes_to_read]);
                window_bytes.reverse();
                // Add em to the window...
                self.m_bitstream_data.window |= u64::from_le_bytes(window_bytes) << window_shift;
                self.m_bitstream_data.window_bits_used += bytes_to_read * 8;
                self.m_bitstream_data.current_stream_byte_position += bytes_to_read;
                self.m_bitstream_data.current_stream_bit_position = 0;
                window_bits_to_read -= bytes_to_read * 8;
            }

            // 3. Read any remaining bits.
            if window_bits_to_read > 0 {
                let current_byte_index = self.m_bitstream_data.current_stream_byte_position;

                let mut bits = self.m_data[current_byte_index];

                // Shift off the excess
                bits >>= 8 - window_bits_to_read;


                let window_shift = 64 - window_bits_to_read - self.m_bitstream_data.window_bits_used;
                self.m_bitstream_data.window |= u64::from(bits) << window_shift;
                self.m_bitstream_data.window_bits_used += window_bits_to_read;
                self.m_bitstream_data.current_stream_bit_position = window_bits_to_read;
                self.m_bitstream_data.current_stream_byte_position = current_byte_index;
            }

            // Write to output.
            let current_memory_position = self.m_bitstream_data.current_memory_bit_position / 8;
            let window_bytes_used = self.m_bitstream_data.window_bits_used.div_ceil(8);
            let next_memory_position = current_memory_position + window_bytes_used;
            let window_value = self.m_bitstream_data.window >> 64 - self.m_bitstream_data.window_bits_used;
            let window_bytes = window_value.to_le_bytes();
            output[current_memory_position..next_memory_position].copy_from_slice(&window_bytes[0..window_bytes_used]);
            self.m_bitstream_data.current_memory_bit_position += self.m_bitstream_data.window_bits_used;

            windows_to_read -= 1;
            self.m_bitstream_data.current_memory_bit_position = next_memory_position * 8;

            if remaining_bits_to_read >= 64 {
                remaining_bits_to_read -= 64;
            } else {
                remaining_bits_to_read = 0;
            }
        }

        self.m_bitstream_data.current_memory_bit_position = 0;
    }


    pub fn read_integer(&mut self, size_in_bits: usize) -> u32 {
        assert!(size_in_bits > 0);
        assert!(size_in_bits <= mem::size_of::<u32>() * 8);
        let mut bytes = [0u8; 4];
        self.read_bits_internal(&mut bytes, size_in_bits);

        match self.m_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => { u32::from_le_bytes(bytes) }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => { u32::from_be_bytes(bytes) }
        }
    }

    pub fn read_identifier(identifier: String) { // param may be wrong.
        unimplemented!()
    }

    pub fn read_point3d(point: &mut int32_point3d, axis_encoding_size_in_bits: u8) {
        unimplemented!()
    }

    pub fn read_quantized_real(min_value: f32, max_value: f32, size_in_bits: u8, exact_midpoint: bool, exact_endpoints: bool) -> f32 {
        unimplemented!()
    }

    pub fn read_qword_internal(size_in_bits: u8) -> u64 {
        unimplemented!()
    }

    pub fn read_secure_address(address: &mut s_transport_secure_address) {
        unimplemented!()
    }

    pub fn read_string(_string: &mut String, max_string_size: u8) {
        unimplemented!()
    }

    pub fn read_string_utf8(_string: &mut String, max_string_size: u8) {
        unimplemented!()
    }

    pub fn read_string_whar(&mut self, write_string: &mut [wchar_t], max_string_size: u8) {
        unimplemented!()
    }

    pub fn read_unit_vector(unit_vector: &mut vector3d, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn read_vector(vector: &mut vector3d, min_value: f32, max_value: f32, step_count_size_in_bits: f32, size_in_bits: u8) {
        unimplemented!()
    }

    // WRITES

    pub fn write_integer(&mut self, value: u32, size_in_bits: usize) {
        match self.m_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                self.write_bits_internal(&value.to_le_bytes(), size_in_bits);
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                self.write_bits_internal(&value.to_be_bytes(), size_in_bits);
            }
        }
    }

    pub fn write_signed_integer(&mut self, value: i32, size_in_bits: usize) {
        match self.m_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                self.write_bits_internal(&value.to_le_bytes(), size_in_bits);
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                self.write_bits_internal(&value.to_be_bytes(), size_in_bits);
            }
        }
    }

    pub fn write_bool(&mut self, value: bool) {
        self.write_integer(if value { 1 } else { 0 }, 1);
    }

    // Be careful using this.
    pub fn write_float(&mut self, value: f32, size_in_bits: usize) {
        match self.m_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                self.write_bits_internal(&value.to_le_bytes(), size_in_bits);
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                self.write_bits_internal(&value.to_be_bytes(), size_in_bits);
            }
        }
    }

    pub fn write_raw_data(&mut self, value: &[u8], size_in_bits: usize) {
        assert!(value.len() >= size_in_bits / 8);
        self.write_bits_internal(value, size_in_bits);
    }

    pub fn write_qword(&mut self, value: u64, size_in_bits: usize) {
        match self.m_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                self.write_bits_internal(&value.to_le_bytes(), size_in_bits);
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                self.write_bits_internal(&value.to_be_bytes(), size_in_bits);
            }
        }
    }

    fn write_value_internal(&mut self, data: &[u8], size_in_bits: usize) {
        self.write_bits_internal(data, size_in_bits);
    }

    fn write_bits_internal(&mut self, data: &[u8], size_in_bits: usize) {
        if data.len() < (size_in_bits as f32 / 8f32).ceil() as usize {
            panic!("Tried to write {size_in_bits} bits but only {} were provided!", (data.len() * 8))
        }

        let bits_available
            = ((self.m_data.len() - self.m_bitstream_data.current_stream_byte_position) * 8)
            - self.m_bitstream_data.current_stream_bit_position;

        // Make a mutable clone of the data to work with.
        let mut data = Vec::from(data);

        // If we were given surplus bits, shift them off.
        let surplus_bits = (data.len() * 8) - size_in_bits;
        left_shift_array(&mut data, surplus_bits);

        let mut remaining_bits_to_write = size_in_bits;

        // 1. Write remaining bits at the current byte.
        let remaining_bits_at_output_position =
            8 - self.m_bitstream_data.current_stream_bit_position;

        if remaining_bits_at_output_position < 8 {
            // of the remaining bits at this byte, how many are we writing?
            let bits_to_write_at_position = min(remaining_bits_to_write, remaining_bits_at_output_position);
            let writing_byte = data[0];
            // TODO: Check this part.
            self.m_data[self.m_bitstream_data.current_stream_byte_position]
                |= writing_byte >> 8 - remaining_bits_at_output_position;

            remaining_bits_to_write -= min(remaining_bits_at_output_position, remaining_bits_to_write);
            // after writing, how many bits are now left at this byte?
            let remaining_bits_at_output_position = remaining_bits_at_output_position - bits_to_write_at_position;
            let more_space_at_current_byte = remaining_bits_at_output_position > 0;

            if !more_space_at_current_byte {
                self.m_bitstream_data.current_stream_bit_position = 0;
                self.m_bitstream_data.current_stream_byte_position += 1;
            } else {
                self.m_bitstream_data.current_stream_bit_position = 8 - remaining_bits_at_output_position;
            }

            left_shift_array(&mut data, bits_to_write_at_position);
        }

        // 2. Write full bytes.
        let bytes_remaining = remaining_bits_to_write / 8;
        for i in 0..bytes_remaining {
            self.m_data[self.m_bitstream_data.current_stream_byte_position] = data[i];

            self.m_bitstream_data.current_stream_byte_position += 1;
            remaining_bits_to_write -= 8;
        }

        // 3. Write remaining bits.
        if remaining_bits_to_write > 0 {
            self.m_data[self.m_bitstream_data.current_stream_byte_position] = data[bytes_remaining];
            self.m_bitstream_data.current_stream_bit_position
                += remaining_bits_to_write;
        }
    }

    pub fn write_identifier(identifier: String) {
        unimplemented!()
    }

    pub fn write_point3d(&mut self, point: &int32_point3d, axis_encoding_size_in_bits: usize) {
        assert!(axis_encoding_size_in_bits > 0 && axis_encoding_size_in_bits <= 32);

        assert!(point.x < 1 << axis_encoding_size_in_bits);
        assert!(point.y < 1 << axis_encoding_size_in_bits);
        assert!(point.z < 1 << axis_encoding_size_in_bits);

        self.write_signed_integer(point.x, axis_encoding_size_in_bits);
        self.write_signed_integer(point.y, axis_encoding_size_in_bits);
        self.write_signed_integer(point.z, axis_encoding_size_in_bits);
    }

    pub fn write_quantized_real(&mut self, value: f32, min_value: f32, max_value: f32, size_in_bits: usize, exact_midpoint: bool, exact_endpoints: bool) {
        assert!(self.writing());
        self.write_signed_integer(quantize_real(value, min_value, max_value, size_in_bits, exact_midpoint, exact_endpoints), size_in_bits);
    }

    pub fn write_secure_address(address: &s_transport_secure_address) {
        unimplemented!()
    }

    pub fn write_string(_string: &String, max_string_size: u32) {
        unimplemented!()
    }

    pub fn write_string_utf8(&mut self, char_string: &String, max_string_size: u32) {
        assert!(self.writing());
        assert!(max_string_size > 0);
        assert!(char_string.len() <= max_string_size as usize);

        for byte in char_string.as_bytes() {
            self.write_value_internal(&[*byte], 8);
        }

        // null terminate
        self.write_value_internal(&0u8.to_ne_bytes(), 8);
    }

    pub fn write_string_wchar(&mut self, value: &String, max_string_size: usize) {
        assert!(self.writing());
        assert!(value.len() <= max_string_size);
        assert!(max_string_size > 0);

        let wchar_string = U16CString::from_str(value).unwrap();
        let characters = wchar_string.as_slice();

        for char in characters {
            match self.m_byte_order {
                e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                    self.write_value_internal(&char.to_le_bytes(), 16);
                }
                e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                    self.write_value_internal(&char.to_be_bytes(), 16);
                }
            }
        }

        // null terminate
        self.write_value_internal(&0u16.to_ne_bytes(), 16);
    }

    pub fn write_unit_vector(unit_vector: &vector3d, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn write_vector(vector: &vector3d, min_value: f32, max_value: f32, step_count_size_in_bits: u32, size_in_bits: u8) {}

    // GUTS

    pub fn append(stream: &c_bitstream) {
        unimplemented!()
    }

    pub fn begin_consistency_check() -> bool {
        unimplemented!()
    }

    pub fn begin_reading() {
        unimplemented!()
    }

    pub fn begin_writing(&mut self, data_size_alignment: u32) {
        self.m_data_size_alignment = data_size_alignment;
        self.reset(e_bitstream_state::_bitstream_state_writing);
    }

    pub fn data_is_untrusted(is_untrusted: bool) {
        unimplemented!()
    }

    pub fn discard_remaining_data() {
        unimplemented!()
    }

    fn encode_qword_to_memory(value: u64, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn overflowed() -> bool {
        unimplemented!()
    }

    pub fn error_occurred() -> bool {
        unimplemented!()
    }

    pub fn reading(&self) -> bool {
        self.m_state == e_bitstream_state::_bitstream_state_reading ||
            self.m_state == e_bitstream_state::_bitstream_state_read_only_for_consistency
    }

    pub fn writing(&self) -> bool {
        self.m_state == e_bitstream_state::_bitstream_state_writing
    }

    pub fn finish_consistency_check() {
        unimplemented!()
    }

    pub fn finish_reading() {
        unimplemented!()
    }

    pub fn finish_writing(&mut self, out_bits_remaining: &mut usize) {
        self.m_state = e_bitstream_state::_bitstream_state_write_finished;
        self.m_data_size_bytes = (((self.m_bitstream_data.current_stream_byte_position * 8) + self.m_bitstream_data.current_stream_bit_position) as f32 / 8f32).ceil() as usize;
        *out_bits_remaining = (8 * self.m_data.len()) - self.m_data_size_bytes;
    }

    pub fn get_current_stream_bit_position() -> u32 {
        unimplemented!()
    }

    pub fn get_space_used_in_bits() -> u32 {
        unimplemented!()
    }

    pub fn get_data(&self, data_length: &mut usize) -> &[u8] {
        assert!(!self.writing());

        *data_length = self.m_data_size_bytes;
        self.m_data
    }

    pub fn push_position() {
        unimplemented!()
    }

    pub fn pop_position(pop: bool) {
        unimplemented!()
    }

    // fn read_accumulator_from_memory(a1: u32) -> u64 {
    //     unimplemented!()
    // }

    fn reset(&mut self, state: e_bitstream_state) {
        self.m_state = state;
        self.m_bitstream_data.current_memory_bit_position = 0;
        self.m_bitstream_data.current_stream_bit_position = 0;
        self.m_position_stack_depth = 0;
        self.__unknown14 = 0;
        self.m_bitstream_data.current_stream_byte_position = 0;
        self.m_bitstream_data.window = 0;
        self.m_bitstream_data.window_bits_used = 0;

        if self.writing() {
            self.__unknown98 = 0;
            self.__unknown9C = 0;
        }
    }

    fn set_data(&mut self, data: &'a mut [u8]) {
        let length = data.len();
        self.m_data = data;
        self.m_data_size_bytes = length;
        self.reset(e_bitstream_state::_bitstream_state_initial);
    }

    fn skip(bits_to_skip: u32) {
        unimplemented!()
    }

    fn would_overflow(size_in_bits: u32) -> bool {
        unimplemented!()
    }

    // fn write_accumulator_to_memory(a1: u64, a2: u32) {
    //     unimplemented!()
    // }

    pub fn axes_compute_reference_internal(
        up: &vector3d,
        forward_reference: &mut vector3d,
        left_reference: &mut vector3d
    ) {
        assert!(assert_valid_real_normal3d(up));

        let v10 = dot_product3d(up, &global_forward3d).abs();
        let v9 = dot_product3d(up, &global_left3d).abs();

        if v10 >= v9 {
            cross_product3d(&global_left3d, up, forward_reference);
        } else {
            cross_product3d(up, &global_forward3d, forward_reference);
        }

        let forward_magnitude = normalize3d(forward_reference);
        assert!(forward_magnitude > k_real_epsilon, "forward_magnitude>k_real_epsilon");

        cross_product3d(up, forward_reference, left_reference);

        let left_magnitude = normalize3d(left_reference);
        assert!(left_magnitude > k_real_epsilon, "left_magnitude>k_real_epsilon");

        assert!(valid_real_vector3d_axes3(forward_reference, left_reference, up)); // Failing
    }

    fn axes_to_angle_internal(forward: &vector3d, up: &vector3d) -> f32 {
        let mut forward_reference: vector3d = vector3d::default();
        let mut left_reference: vector3d = vector3d::default();
        c_bitstream::axes_compute_reference_internal(up, &mut forward_reference, &mut left_reference);
        arctangent(dot_product3d(&left_reference, &forward), dot_product3d(&forward_reference, &forward))
    }

    pub fn write_axes(
        &mut self,
        forward: &vector3d,
        up: &vector3d,
    ) {
        assert!(assert_valid_real_normal3d(up));
        assert!(assert_valid_real_normal3d(forward));

        let mut dequantized_up: vector3d = vector3d::default();

        let i_abs = (up.i - global_up3d.i).abs();
        let j_abs = (up.j - global_up3d.j).abs();
        let k_abs = (up.k - global_up3d.k).abs();

        // Compare the a4 vector with global_up3d
        if i_abs > k_real_epsilon
            || j_abs > k_real_epsilon
            || k_abs > k_real_epsilon
        {
            let quantized_up = quantize_unit_vector3d(up);
            self.write_bool(false); // up-is-global-up3d
            self.write_integer(quantized_up as u32, 19); // correct, as is quantized_up
            dequantize_unit_vector3d(quantized_up, &mut dequantized_up);
        } else {
            self.write_bool(true); // up-is-global-up3d
            dequantized_up = global_up3d.clone();
        }

        // angle may be bad
        let angle = c_bitstream::axes_to_angle_internal(forward, &dequantized_up);
        // this is fine
        self.write_quantized_real(angle, -3.1415927, 3.1415927, 8, true, false);
    }

    // not from blam
    pub fn get_current_offset(&self) -> usize {
        self.m_bitstream_data.current_stream_byte_position
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bits_internal() {
        let data: &mut [u8] = &mut [
            0b10101010, 0b11001100, 0b11110000
        ];
        let mut bitstream = c_bitstream::new(data, e_bitstream_byte_order::_bitstream_byte_order_little_endian);

        let mut output = [0u8; 2]; // Output buffer for storing results

        // Read 5 bits
        bitstream.read_bits_internal(&mut output, 5);
        assert_eq!(output[0], 0b10101);

        output = [0u8; 2];
        // Read 9 bits
        bitstream.read_bits_internal(&mut output, 9);
        assert_eq!(output[0], 0b10110011);
        assert_eq!(output[1], 0b00000000);
    }

    #[test]
    fn test_read_integers() {
        let data: &mut [u8] = &mut [
            0x03, 0x29, 0x48, 0xFF, 0xAB, 0x30, 0x92, 0x49, 0xFF
        ];

        let mut bitstream = c_bitstream::new(data, e_bitstream_byte_order::_bitstream_byte_order_little_endian);

        assert_eq!(bitstream.read_integer(10), 12);
        assert_eq!(bitstream.read_integer(3), 5);
        assert_eq!(bitstream.read_integer(24), 2695157);
        assert_eq!(bitstream.read_integer(32), 1712474431)
    }
}

// Not from blam
fn left_shift_array(data: &mut Vec<u8>, shift: usize) {
    if shift == 0 || data.is_empty() {
        return;
    }

    let len = data.len();
    let byte_shift = shift / 8;
    let bit_shift = shift % 8;

    // Shift bytes
    if byte_shift != 0 {
        for i in 0..len {
            if i + byte_shift < len {
                data[i] = data[i + byte_shift]
            } else {
                data[i] = 0;
            }
        }
    }

    // Shift bits
    data[0] = data[0] << bit_shift;
    for i in 1..(len - byte_shift) {
        // use a short for shifting
        let current_byte = data[i];
        let shift_window = current_byte as u16;
        let shift_window = shift_window << bit_shift;
        let [carry_bits, shifted_byte] = shift_window.to_be_bytes();
        data[i - 1] |= carry_bits;
        data[i] = shifted_byte;
    }
}