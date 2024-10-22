mod bitstream_reader;
mod bitstream_writer;

pub use bitstream_reader::c_bitstream_reader;
pub use bitstream_writer::c_bitstream_writer;

#[derive(Default, PartialEq, Eq, Debug)]
pub enum e_bitstream_byte_order
{
    #[default]
    _bitstream_byte_order_little_endian,
    _bitstream_byte_order_big_endian
}

#[derive(Default, PartialEq, Eq, Debug)]
pub enum e_bitstream_state
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

pub const k_bitstream_maximum_position_stack_size: usize = 4;

#[derive(Default)]
pub struct s_bitstream_data {
    current_memory_bit_position: usize,
    current_stream_bit_position: usize,
    window: u64,
    window_bits_used: usize,
    current_stream_byte_position: usize, // aka current_memory_byte_position
}