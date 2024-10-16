use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_banhammer_messages, s_blf_chunk_end_of_file, s_blf_chunk_start_of_file};
use blf_lib::blf_file;

blf_file! {
    pub struct matchmaking_banhammer_messages {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        bhms: s_blf_chunk_banhammer_messages,
        _eof: s_blf_chunk_end_of_file,
    }
}