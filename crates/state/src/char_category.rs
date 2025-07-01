use regex::{Error, Regex};

const NON_ASCII_SINGLE_CASE_WORD_CHAR: Regex = Regex::new(r"[\u00df\u0587\u0590-\u05f4\u0600-\u06ff\u3040-\u309f\u30a0-\u30ff\u3400-\u4db5\u4e00-\u9fcc\uac00-\ud7af]").unwrap();

/// TODO:
const WORD_CHAR: Result<Regex, Error> = Regex::new(r"[\\p{Alphabetic}\\p{Number}_]");

/// 字符类别
pub enum CharCategory {
    Word,
    Space,
    Other,
}

fn has_word_char(str: String) -> bool {
    if let Ok(_word_char) = WORD_CHAR {
        return _word_char.is_match(str.as_str());
    }

    let _word = Regex::new(r"\w").unwrap();

    for ch in str.chars().into_iter() {
        if _word.is_match(*ch) ||
            *ch as u32 > 0x80 &&
                (*ch.to_lowercase() != *ch.to_uppercase() || NON_ASCII_SINGLE_CASE_WORD_CHAR.is_match(*ch)) {
            return true;
        }
    }


    return false;
}

pub fn create_categorizer(word_chars: String) -> impl Fn() {
    return move |char: String| -> CharCategory {
        if !Regex::new(r"\S").unwrap().captures(char.as_str()) {
            return CharCategory::Space;
        }

        if has_word_char(char) {
            return CharCategory::Word;
        }

        for word in word_chars.chars().into_iter() {
            if char.contains(word) {
                return CharCategory::Word;
            }
        }

        return CharCategory::Other;
    };
}