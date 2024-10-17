use blf_lib::blf_chunk;

blf_chunk!(
    #[Signature("motd")]
    #[Version(1.1)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_message_of_the_day
    {
        motd_length: u32,
        motd_message: String,
    }
);

impl s_blf_chunk_message_of_the_day {
    pub fn new(motd_message: String) -> s_blf_chunk_message_of_the_day {
        let mut motd = s_blf_chunk_message_of_the_day::default();
        motd.set_message(motd_message);
        motd
    }

    pub fn set_message(&mut self, motd_message: String) {
        self.motd_message = motd_message;
        self.motd_length = self.motd_message.len() as u32;
    }

    pub fn get_message(&self) -> &String {
        &self.motd_message
    }
}
//
// impl SerializableBlfChunk for s_blf_chunk_message_of_the_day {
//     fn encode_body(&mut self, previously_written: &Vec<u8>) -> Vec<u8> {
//         let mut encoded = Vec::<u8>::with_capacity(self.motd_message.len() + 0x4);
//         let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();
//
//         encoded.append(&mut bincode::encode_to_vec(self.motd_length, config).unwrap());
//         for x in self.motd_message.bytes() {
//             encoded.append(&mut bincode::encode_to_vec(x as u8, bincode::config::standard()).unwrap());
//         }
//
//         encoded
//     }
//
//     fn decode_body(&mut self, buffer: &[u8]) {
//         let config = bincode::config::standard().with_big_endian().with_fixed_int_encoding();
//
//         self.motd_length = bincode::decode_from_slice(buffer, config).unwrap().0;
//         self.motd_message = String::from_utf8(Vec::from(&buffer[4..])).unwrap();
//
//     }
// }