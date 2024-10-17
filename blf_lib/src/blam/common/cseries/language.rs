pub const k_language_suffix_english: &str = "en";
pub const k_language_suffix_japanese: &str = "jpn";
pub const k_language_suffix_german: &str = "de";
pub const k_language_suffix_french: &str = "fr";
pub const k_language_suffix_spanish: &str = "sp";
pub const k_language_suffix_mexican: &str = "mx";
pub const k_language_suffix_italian: &str = "it";
pub const k_language_suffix_korean: &str = "kor";
pub const k_language_suffix_chinese_traditional: &str = "cht";
pub const k_language_suffix_chinese_simplified: &str = "chs";
pub const k_language_suffix_portuguese: &str = "pt";
pub const k_language_suffix_polish: &str = "pl";


pub const k_language_suffixes: [&str; 12] = [
    k_language_suffix_english,
    k_language_suffix_japanese,
    k_language_suffix_german,
    k_language_suffix_french,
    k_language_suffix_spanish,
    k_language_suffix_mexican,
    k_language_suffix_italian,
    k_language_suffix_korean,
    k_language_suffix_chinese_traditional,
    k_language_suffix_chinese_simplified,
    k_language_suffix_portuguese,
    k_language_suffix_polish,
];

pub fn get_language_string(language_code: &str) -> &str {
    match language_code {
        k_language_suffix_english => "English",
        k_language_suffix_japanese => "Japanese",
        k_language_suffix_german => "German",
        k_language_suffix_french => "French",
        k_language_suffix_spanish => "Spanish",
        k_language_suffix_mexican => "Mexican",
        k_language_suffix_italian => "Italian",
        k_language_suffix_korean => "Korean",
        k_language_suffix_chinese_traditional => "Chinese (Traditional)",
        k_language_suffix_chinese_simplified => "Chinese (Simplified)",
        k_language_suffix_portuguese => "Portuguese",
        k_language_suffix_polish => "Polish",
        _ => panic!("Invalid language code {}!", language_code)
    }
}