use std::ffi::{c_char};
use std::u32;
use blf_lib::blf_chunk;
use blf_lib::types::byte_limited_utf8_string::ByteLimitedUTF8String;

const MAX_MATCHMAKING_TIP_COUNT: usize = 32usize;
const TIP_LENGTH: usize = 0x100;

blf_chunk!(
    #[Signature("mmtp")]
    #[Version(1.1)]
    #[PackedSerialize(1, BigEndian)]
    pub struct s_blf_chunk_matchmaking_tips
    {
        tip_count: u32,
        pub tips: Vec<ByteLimitedUTF8String<TIP_LENGTH>> // UTF bytes,
    }
);

impl s_blf_chunk_matchmaking_tips {
    pub fn get_tips(&self) -> Vec<String> {
        self.tips.iter().map(|tip|tip.get_string()).collect()
    }

    fn set_tips(&mut self, tips: Vec<String>) -> Result<(), String> {
        if tips.len() > MAX_MATCHMAKING_TIP_COUNT {
            return Err(format!("Too many tips! {}/{MAX_MATCHMAKING_TIP_COUNT}", tips.len()))
        }

        self.tips = Vec::with_capacity(tips.len());
        for tip in tips.iter() {
            let tip = ByteLimitedUTF8String::<TIP_LENGTH>::from_string(tip);

            if !tip.is_ok() {
                return Err(format!("Tip: {}", tip.unwrap_err()))
            }

            let tip = tip?;

            self.tips.push(tip);
        }
        self.tip_count = self.tips.len() as u32;
        Ok(())
    }

    pub fn create(tips: Vec<String>) -> s_blf_chunk_matchmaking_tips {
        let mut new = Self::default();
        new.set_tips(tips);
        new
    }
}