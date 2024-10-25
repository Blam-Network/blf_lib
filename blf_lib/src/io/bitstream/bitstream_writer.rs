
use std::cmp::min;
use widestring::U16CString;
use blf_lib::blam::common::math::real_math::{assert_valid_real_normal3d, cross_product3d, dequantize_unit_vector3d, dot_product3d, k_real_epsilon, global_forward3d, global_left3d, global_up3d, normalize3d, valid_real_vector3d_axes3, arctangent, quantize_normalized_vector3d, k_pi};
use crate::blam::common::math::integer_math::int32_point3d;
use crate::blam::common::math::real_math::{quantize_real, vector3d};
use crate::blam::common::networking::transport::transport_security::s_transport_secure_address;
use crate::io::bitstream::{e_bitstream_byte_order, e_bitstream_state, k_bitstream_maximum_position_stack_size, s_bitstream_data};

pub struct c_bitstream_writer<'a>
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

impl<'a> c_bitstream_writer<'a> {
    pub fn new(data: &mut [u8], byte_order: e_bitstream_byte_order) -> c_bitstream_writer {
        let length = data.len();
        c_bitstream_writer {
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

    pub fn append(stream: &c_bitstream_writer) {
        unimplemented!()
    }

    pub fn begin_consistency_check() -> bool {
        unimplemented!()
    }

    pub fn begin_reading(&mut self) {
        panic!("Bitstream writer cannot read!")
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
        unreachable!()
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
        c_bitstream_writer::axes_compute_reference_internal(up, &mut forward_reference, &mut left_reference);
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

        if i_abs > k_real_epsilon
            || j_abs > k_real_epsilon
            || k_abs > k_real_epsilon
        {
            let quantized_up = quantize_normalized_vector3d(up);
            self.write_bool(false); // up-is-global-up3d
            self.write_integer(quantized_up as u32, 19);
            dequantize_unit_vector3d(quantized_up, &mut dequantized_up);
        } else {
            self.write_bool(true); // up-is-global-up3d
            dequantized_up = global_up3d.clone();
        }

        let forward_angle = c_bitstream_writer::axes_to_angle_internal(forward, &dequantized_up);
        self.write_quantized_real(forward_angle, -k_pi, k_pi, 8, true, false);
    }

    // not from blam
    pub fn get_current_offset(&self) -> usize {
        self.m_bitstream_data.current_stream_byte_position
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