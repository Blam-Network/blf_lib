// This module is based on ManagedDonkey's bitstream module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/memory/bitstream.cpp

// Changes:
// - We don't use pointer arithmetic internally, as such m_data_max is removed.
// - The empty constructor is removed, function overloads are unsupported in Rust.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]


use libc::wchar_t;
use crate::blam::math::integer_math::int32_point3d;
use crate::blam::math::real_math::vector3d;
use crate::blam::networking::transport::transport_security::s_transport_secure_address;

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

struct c_bitstream<'a>
{
    m_data: &'a [u8],
    // m_data_max: u32, REMOVED
    m_data_size_bytes: usize,
    m_data_size_alignment: u32,
    m_state: e_bitstream_state,
    __unknown14: u32, // might be debug mode
    m_bitstream_data: s_bitstream_data,
    m_position_stack_depth: u32,
    __unknown34: u32,
    m_position_stack: [s_bitstream_data; k_bitstream_maximum_position_stack_size],
    __unknown98: u32,
    __unknown9C: u32,
}

impl<'a> c_bitstream<'a> {

    // pub fn new() {}

    pub fn new(data: &mut [u8]) -> c_bitstream {
        c_bitstream {
            m_data: data,
            m_data_size_bytes: data.len(),
            m_data_size_alignment: 1,
            m_state: e_bitstream_state::_bitstream_state_initial,
            __unknown14: Default::default(),
            m_bitstream_data: Default::default(),
            m_position_stack_depth: 0,
            __unknown34: Default::default(),
            m_position_stack: Default::default(),
            __unknown98: Default::default(),
            __unknown9C: Default::default(),
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
        let mut bits_remaining = size_in_bits;

        let mut current_memory_position = 0;
        let mut current_stream_position = self.m_bitstream_data.current_stream_byte_position;
        let mut next_memory_position = 0;
        let mut next_stream_position = 0;
        let end_memory_position = output.len();
        let end_stream_position = self.m_data.len();
        let remaining_stream_bytes = end_stream_position - current_stream_position;

        let size_in_bytes = (size_in_bits / 8) + 1;
        if end_memory_position < size_in_bytes {
            panic!("Tried to read {size_in_bits} bits into a {end_memory_position} byte buffer!")
        }

        if remaining_stream_bytes < size_in_bytes {
            panic!("Tried to read {size_in_bits} bits but the stream only has {remaining_stream_bytes} bytes left!")
        }

        // Process full 64-bit chunks
        while bits_remaining >= 64 {
            next_stream_position = current_stream_position + 8;
            next_memory_position = current_memory_position + 8;

            // Add bytes to the window...
            if next_stream_position <= end_stream_position {
                let window_bytes = &self.m_data[current_stream_position..next_stream_position];
                self.m_bitstream_data.window = u64::from_le_bytes(window_bytes.try_into().unwrap());
                self.m_bitstream_data.current_stream_byte_position += 8;
            } else {
                // Handle case where data runs out before we can read 8 bytes
                let bytes_to_read = end_stream_position - self.m_bitstream_data.current_stream_byte_position;
                let mut window_bytes = [0u8; 8];
                window_bytes[..bytes_to_read].copy_from_slice(&self.m_data[self.m_bitstream_data.current_stream_byte_position..self.m_bitstream_data.current_stream_byte_position + size_of::<u64>()]);
                self.m_bitstream_data.window = u64::from_le_bytes(window_bytes) << (64 - (bytes_to_read * 8));
                self.m_bitstream_data.current_stream_byte_position += bytes_to_read;
            }

            // The window is full, add it to the output buffer.
            let window_value = self.m_bitstream_data.window >> (64 - bits_remaining);
            output[(size_in_bits / 8 + 1) - (bits_remaining / 8)..(size_in_bits / 8 + 1) - ((bits_remaining / 8) + 8)].copy_from_slice(&window_value.to_le_bytes());
            bits_remaining -= 64;
            self.m_bitstream_data.current_memory_bit_position += 64;
            self.m_bitstream_data.current_stream_bit_position += 64;
            self.m_bitstream_data.window_bits_used = 0;
            current_memory_position = next_memory_position;
            current_stream_position = next_stream_position;
        }

        if bits_remaining >= 8 {
            let bytes_remaining = bits_remaining / 8;

            next_memory_position = current_memory_position + bytes_remaining;
            next_stream_position = current_stream_position + bytes_remaining;

            let window_shift = 64 - (bytes_remaining * 8);

            // grab as many bytes as we can.
            let read_bytes = &self.m_data[current_stream_position..next_stream_position];
            let mut window_bytes: [u8; 8] = [0; 8];
            window_bytes[0..bytes_remaining].copy_from_slice(read_bytes);
            self.m_bitstream_data.window = u64::from_le_bytes(window_bytes) << window_shift;

            bits_remaining -= bytes_remaining * 8;

            if bits_remaining == 0 {
                let window_value = self.m_bitstream_data.window >> window_shift;
                output[current_memory_position..next_memory_position].copy_from_slice(&window_value.to_le_bytes()[0..bytes_remaining]);
                self.m_bitstream_data.window = 0;
                self.m_bitstream_data.window_bits_used = 0;
                current_memory_position = next_memory_position;
            } else {
                self.m_bitstream_data.window_bits_used = bytes_remaining * 8;
            }

            self.m_bitstream_data.current_memory_bit_position += bytes_remaining * 8;
            self.m_bitstream_data.current_stream_bit_position += bytes_remaining * 8;
            current_stream_position = next_stream_position;
        }

        // Handle remaining bits if any
        if bits_remaining > 0 {
            next_memory_position = next_memory_position + 1;
            next_stream_position = current_stream_position + 1;

            self.m_bitstream_data.window_bits_used += bits_remaining;
            let window_bytes_used = (self.m_bitstream_data.window_bits_used / 8) + 1;

            let window_shift = 64 - self.m_bitstream_data.window_bits_used;

            // Read the next partial window if there's still data
            let final_byte = self.m_data[current_stream_position];
            self.m_bitstream_data.window |= u64::from(final_byte) << (window_shift - 1);

            // Store the remaining bits
            let window_value = self.m_bitstream_data.window >> window_shift;
            output[current_memory_position..next_memory_position].copy_from_slice(&window_value.to_le_bytes()[0..window_bytes_used]);
            self.m_bitstream_data.window = 0;
            self.m_bitstream_data.window_bits_used = 0;
            self.m_bitstream_data.current_memory_bit_position += bits_remaining;
            self.m_bitstream_data.current_stream_bit_position += bits_remaining;
            current_memory_position = next_memory_position;
            current_stream_position = next_stream_position;
        }

        self.m_bitstream_data.current_stream_byte_position = next_stream_position;
    }


    pub fn read_integer(size_in_bits: u8) -> u32 {
        unimplemented!()
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

    pub fn read_string_whar(_string: &mut [wchar_t], max_string_size: u8) {
        unimplemented!()
    }

    pub fn read_unit_vector(unit_vector: &mut vector3d, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn read_vector(vector: &mut vector3d, min_value: f32, max_value: f32, step_count_size_in_bits: f32, size_in_bits: u8) {
        unimplemented!()
    }

    // WRITES

    pub fn write_bool(value: bool) {
        unimplemented!()
    }

    pub fn write_integer(value: u32, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn write_raw_data(value: &[u8], size_in_bits: u8) {
        unimplemented!()
    }

    pub fn write_signed_integer(value: i32, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn write_qword(value: u64, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn write_bits_internal(data: &[u8], size_in_bits: u32) {
        unimplemented!()
    }

    pub fn write_identifier(identifier: String) {
        unimplemented!()
    }

    pub fn write_point3d(point: int32_point3d, axis_encoding_size_in_bits: u8) {
        unimplemented!()
    }

    pub fn write_quantized_real(value: f32, min_value: f32, max_value: f32, size_in_bits: u8, exact_midpoint: bool, exact_endpoints: bool) {
        unimplemented!()
    }

    pub fn write_secure_address(address: &s_transport_secure_address) {
        unimplemented!()
    }

    pub fn write_string(_string: &String, max_string_size: u32) {
        unimplemented!()
    }

    pub fn write_string_utf8(_string: &String, max_string_size: u32) {
        unimplemented!()
    }

    pub fn write_string_whar(_string: &[wchar_t], max_string_size: u32) {
        unimplemented!()
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

    pub fn begin_writing(data_size_alignment: u32) {
        unimplemented!()
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

    pub fn finish_writing(out_bits_remaining: &mut u32) {
        unimplemented!()
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

    fn read_accumulator_from_memory(a1: u32) -> u64 {
        unimplemented!()
    }

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

    fn set_data(&mut self, data: &'a [u8]) {
        self.m_data = data;
        self.m_data_size_bytes = data.len();
        self.reset(e_bitstream_state::_bitstream_state_initial);
    }

    fn skip(bits_to_skip: u32) {
        unimplemented!()
    }

    fn would_overflow(size_in_bits: u32) -> bool {
        unimplemented!()
    }

    fn write_accumulator_to_memory(a1: u64, a2: u32) {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_reading_state() {
        let mut data = vec![0u8; 64]; // Create a buffer with 64 bytes
        // Pre-fill some data for reading
        data[..8].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);

        let mut bitstream = c_bitstream::new(&mut data);

        bitstream.reset(e_bitstream_state::_bitstream_state_reading); // Reset to reading state

        // Assert that the internal state is set correctly
        assert_eq!(bitstream.m_state, e_bitstream_state::_bitstream_state_reading);
        assert_eq!(bitstream.m_bitstream_data.current_memory_bit_position, 64);
        assert_eq!(bitstream.m_bitstream_data.current_stream_bit_position, 0);
        assert_eq!(bitstream.m_position_stack_depth, 0);
        assert!(bitstream.__unknown14 == 0);
        assert_eq!(bitstream.m_bitstream_data.current_stream_byte_position, 8); // Should point to the next byte after the read
        assert!(bitstream.m_bitstream_data.window != 0); // Check that the window has been populated
    }

    #[test]
    fn test_read_bits_internal() {
        // Setup: Create a bitstream with known data.
        let data: &mut [u8] = &mut [
            0b10101010, 0b11110000, // First 16 bits (2 bytes)
            0b00110011, 0b00001111, // Next 16 bits (2 bytes)
        ];

        let mut bitstream = c_bitstream::new(data);

        // Set the state to reading (assumes the state enum is implemented)
        bitstream.reset(e_bitstream_state::_bitstream_state_reading);

        // Prepare a buffer to read the bits into
        let mut buffer: [u8; 4] = [0; 4]; // Enough space for 32 bits (4 bytes)

        // Read 32 bits
        bitstream.read_bits_internal(&mut buffer, 32);

        // Check the output
        let expected: [u8; 4] = [
            0b10101010, 0b11110000, // First 16 bits (2 bytes)
            0b00110011, 0b00001111, // Next 16 bits (2 bytes)
        ];

        // Assert that the buffer matches the expected output
        assert_eq!(buffer, expected);

        // Optionally, verify the internal state after reading
        assert_eq!(bitstream.m_bitstream_data.current_memory_bit_position, 32);
        assert_eq!(bitstream.m_bitstream_data.current_stream_bit_position, 32);
    }

    #[test]

    fn test_read_bits_internal_multiple_reads() {
        // Sample data that will provide sufficient bits for testing
        let data: &mut [u8] = &mut [
            0b10101010, 0b11001100, 0b11110000
        ];
        let mut bitstream = c_bitstream::new(data);

        let mut output = [0u8; 2]; // Output buffer for storing results

        // Read 5 bits
        bitstream.read_bits_internal(&mut output, 5);
        assert_eq!(output[0], 0b10101); // Expect the first 5 bits to be 0b10101

        output = [0u8; 2];
        // Read 9 bits
        bitstream.read_bits_internal(&mut output, 9);
        assert_eq!(output[0], 0b01011001); // Expect the next 9 bits to be 0b110011001
        assert_eq!(output[1], 0b00000001); // Expect the next 9 bits to be 0b110011001

    }
}