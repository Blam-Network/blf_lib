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