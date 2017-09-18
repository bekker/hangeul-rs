//! [![](http://meritbadge.herokuapp.com/hangeul)](https://crates.io/crates/hangeul)
//! 
//! [View on GitHub](https://github.com/bekker/hangeul-rs)
//!
//! Korean alphabet manipulation library for Rust.
//!
//! It is lightweight, without any unicode libraries.
//!
//! ```toml
//! [dependencies]
//! hangeul = "0.1.3"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! extern crate hangeul;
//!
//! fn main() {
//!     let subjective = "피카츄";
//!     let sub_josa = match hangeul::ends_with_jongseong(subjective).unwrap() {
//!         true => "이",
//!         false => "가"
//!     };
//!     let sentence = format!("야생의 {}{} 나타났다!", subjective, sub_josa);
//!     println!("{}", sentence); // 야생의 피카츄가 나타났다!
//!     print_choseong(&sentence); // ㅇㅅㅇ ㅍㅋㅊㄱ ㄴㅌㄴㄷ!
//! }
//!
//! fn print_choseong(s:&str) {
//!     for c in s.chars() {
//!         print!("{}", hangeul::get_choseong(c).unwrap_or(c));
//!     }
//!     print!("\n");
//! }
//! ```
//!
//! ## Why not 'Hangul'?
//! 'Hangul' is from old romanization system - McCune–Reischauer system.
//!
//! 'Hangeul' is official in South Korea, since 2000
//!
//! ## License
//! Distributed under MIT License
//!

use std::fmt;
use std::error;

#[cfg(test)]
mod tests {
    use super::JAMO_TO_CHOSEONG;
    use super::JAMO_TO_JONGSEONG;
    use super::CHOSEONG_TO_JAMO;
    use super::JONGSEONG_TO_JAMO;

    #[test]
    fn jamo_table_test() {
        for x in 0..JAMO_TO_CHOSEONG.len() {
            let choseong_index = JAMO_TO_CHOSEONG[x];
            if choseong_index == -1 {
                continue;
            }

            let jamo_index = CHOSEONG_TO_JAMO[choseong_index as usize] as usize;
            assert_eq!(jamo_index, x);
        }

        for x in 0..JAMO_TO_JONGSEONG.len() {
            let jongseong_index = JAMO_TO_JONGSEONG[x];
            if jongseong_index == -1 {
                continue;
            }

            let jamo_index = JONGSEONG_TO_JAMO[jongseong_index as usize] as usize;
            assert_eq!(jamo_index, x);
        }
    }
}

#[derive(Debug)]
pub enum HangeulError {
    NotSyllable,
    Uncomposable,
}

impl fmt::Display for HangeulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HangeulError::NotSyllable => write!(f, "HangeulError: Not correct Hangeul syllable"),
            HangeulError::Uncomposable => write!(f, "HangeulError: Uncomposable")
        }
    }
}

impl error::Error for HangeulError {
    fn description(&self) -> &str {
        match *self {
            HangeulError::NotSyllable => "HangeulError: Not correct Hangeul syllable",
            HangeulError::Uncomposable => "HangeulError: Uncomposable"
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None
        }
    }
}

const JAMO_TO_CHOSEONG: [i32; 30] = [
    0x00,  // ㄱ 0x3131
    0x01,  // ㄲ
    -1,    // ㄳ
    0x02,  // ㄴ
    -1,    // ㄵ
    -1,    // ㄶ
    0x03,  // ㄷ
    0x04,  // ㄸ
    0x05,  // ㄹ
    -1,    // ㄺ
    -1,    // ㄻ
    -1,    // ㄼ
    -1,    // ㄽ
    -1,    // ㄾ
    -1,    // ㄿ
    -1,    // ㅀ
    0x06,  // ㅁ
    0x07,  // ㅂ
    0x08,  // ㅃ
    -1,    // ㅄ
    0x09,  // ㅅ
    0x0A,  // ㅆ
    0x0B,  // ㅇ
    0x0C,  // ㅈ
    0x0D,  // ㅉ
    0x0E,  // ㅊ
    0x0F,  // ㅋ
    0x10,  // ㅌ
    0x11,  // ㅍ
    0x12,  // ㅎ 0x314E
];

const JAMO_TO_JONGSEONG: [i32; 30] = [
    0x00,  // ㄱ 0x3131
    0x01,  // ㄲ
    0x02,  // ㄳ
    0x03,  // ㄴ
    0x04,  // ㄵ
    0x05,  // ㄶ
    0x06,  // ㄷ
    -1,    // ㄸ
    0x07,  // ㄹ
    0x08,  // ㄺ
    0x09,  // ㄻ
    0x0A,  // ㄼ
    0x0B,  // ㄽ
    0x0C,  // ㄾ
    0x0D,  // ㄿ
    0x0E,  // ㅀ
    0x0F,  // ㅁ
    0x10,  // ㅂ
    -1,    // ㅃ
    0x11,  // ㅄ
    0x12,  // ㅅ
    0x13,  // ㅆ
    0x14,  // ㅇ
    0x15,  // ㅈ
    -1,    // ㅉ
    0x16,  // ㅊ
    0x17,  // ㅋ
    0x18,  // ㅌ
    0x19,  // ㅍ
    0x1A,  // ㅎ 0x314E
];

// These tables are for converting to compatible jamo
const CHOSEONG_TO_JAMO: [u32; 19] = [
    0x00, // ㄱ
    0x01, // ㄲ
    0x03, // ㄴ
    0x06, // ㄷ
    0x07, // ㄸ
    0x08, // ㄹ
    0x10, // ㅁ
    0x11, // ㅂ
    0x12, // ㅃ
    0x14, // ㅅ
    0x15, // ㅆ
    0x16, // ㅇ
    0x17, // ㅈ
    0x18, // ㅉ
    0x19, // ㅊ
    0x1A, // ㅋ
    0x1B, // ㅌ
    0x1C, // ㅍ
    0x1D, // ㅎ
];

const JONGSEONG_TO_JAMO: [u32; 27] = [
    0x00, // ㄱ
    0x01, // ㄲ
    0x02, // ㄳ
    0x03, // ㄴ
    0x04, // ㄵ
    0x05, // ㄶ
    0x06, // ㄷ
    0x08, // ㄹ
    0x09, // ㄺ
    0x0A, // ㄻ
    0x0B, // ㄼ
    0x0C, // ㄽ
    0x0D, // ㄾ
    0x0E, // ㄿ
    0x0F, // ㅀ
    0x10, // ㅁ
    0x11, // ㅂ
    0x13, // ㅄ
    0x14, // ㅅ
    0x15, // ㅆ
    0x16, // ㅇ
    0x17, // ㅈ
    0x19, // ㅊ
    0x1A, // ㅋ
    0x1B, // ㅌ
    0x1C, // ㅍ
    0x1D, // ㅎ
];

const SYLLABLE_START: u32 = 0xAC00;
const SYLLABLE_END: u32 = 0xD7AF;
const JAMO_START: u32 = 0x3131;
const JAMO_END: u32 = 0x3163;
const JAEUM_START: u32 = 0x3131;
const JAEUM_END: u32 = 0x314E;
const MOEUM_START: u32 = 0x314F;
const MOEUM_END: u32 = 0x3163;

fn _is_syllable(code:u32) -> bool {
    (code >= SYLLABLE_START && code <= SYLLABLE_END)
}

/// Check if the syllable is correct Hangeul syllable
pub fn is_syllable(c:char) -> bool {
    let code = c as u32;
    _is_syllable(code)
}

fn syllable_to_u32(c:char) -> Result<u32, HangeulError> {
    let code = c as u32;
    if _is_syllable(code) {
        Ok(code)
    } else {
        Err(HangeulError::NotSyllable)
    }
}

/// Get choseong (top) of the syllable as compatible jamo
pub fn get_choseong(c:char) -> Result<char, HangeulError> {
    let code = try!(syllable_to_u32(c));
    let x = (code - SYLLABLE_START) / 21 / 28;
    std::char::from_u32(CHOSEONG_TO_JAMO[x as usize] + JAMO_START).ok_or(HangeulError::NotSyllable)
}

/// Alias for get_choseong
pub use self::get_choseong as get_top;

/// Get jungseong (middle) of the syllable as compatible jamo
pub fn get_jungseong(c:char) -> Result<char, HangeulError> {
    let code = try!(syllable_to_u32(c));
    std::char::from_u32(((code - SYLLABLE_START) % (21 * 28)) / 28 + MOEUM_START).ok_or(HangeulError::NotSyllable)
}

/// Alias for get_jungseong
pub use self::get_jungseong as get_middle;

/// Get jongseong (bottom) of the syllable as compatible jamo
pub fn get_jongseong(c:char) -> Result<Option<char>, HangeulError> {
    let code = try!(syllable_to_u32(c));
    // x should be i32, can be negative
    let x:i32 = (code - SYLLABLE_START) as i32 % 28 - 1;
    if x >= 0 {
        std::char::from_u32(JONGSEONG_TO_JAMO[x as usize] + JAMO_START)
            .map_or(Err(HangeulError::NotSyllable), |c| Ok(Some(c)))
    } else {
        Ok(None)
    }
}

/// Alias for get_jongseong
pub use self::get_jongseong as get_bottom;

/// Check if the syllable has jongseong (bottom)
pub fn has_jongseong(c:char) -> Result<bool, HangeulError> {
    let code = try!(syllable_to_u32(c));
    Ok((code - SYLLABLE_START) % 28 != 0)
}

/// Alias for has_jongseong
pub use self::has_jongseong as has_bottom;

/// Check if the end syllable of the string has jongseong (bottom)
pub fn ends_with_jongseong(s:&str) -> Result<bool, HangeulError> {
    let c = match s.chars().last() {
        Some(x) => x,
        None => return Err(HangeulError::NotSyllable),
    };
    has_jongseong(c)
}

/// Alias for ends_with_jongseong
pub use self::ends_with_jongseong as ends_with_bottom;

/// Check if the char is compatible jamo
pub fn is_jamo(c:char) -> bool {
    let code = c as u32;
    (code >= JAMO_START && code <= JAMO_END)
}

fn _is_jaeum(code:u32) -> bool {
    (code >= JAEUM_START && code <= JAEUM_END)
}

/// Check if the char is compatible jaeum
pub fn is_jaeum(c:char) -> bool {
    let code = c as u32;
    _is_jaeum(code)
}

/// Alias for is_jaeum
pub use self::is_jaeum as is_consonant;

fn _is_moeum(code:u32) -> bool {
    (code >= MOEUM_START && code <= MOEUM_END)
}

/// Check if the char is compatible moeum
pub fn is_moeum(c:char) -> bool {
    let code = c as u32;
    _is_moeum(code)
}

/// Alias for is_moeum
pub use self::is_moeum as is_vowel;

fn _is_choseong(code:u32) -> bool {
    _is_jaeum(code) && (JAMO_TO_CHOSEONG[(code - JAEUM_START) as usize] != -1)
}

/// Check if the char is compatible jamo which can be a choseong (top)
pub fn is_choseong(c:char) -> bool {
    let code = c as u32;
    _is_choseong(code)
}

/// Alias for is_choseong
pub use self::is_choseong as is_top;

fn _is_jongseong(code:u32) -> bool {
    _is_jaeum(code) && (JAMO_TO_JONGSEONG[(code - JAEUM_START) as usize] != -1)
}

/// Check if the char is compatible jamo which can be a jongseong (bottom)
pub fn is_jongseong(c:char) -> bool {
    let code = c as u32;
    _is_jongseong(code)
}

/// Alias for is_jongseong
pub use self::is_jongseong as is_bottom;

/// Decompose a Hangeul syllable into compatible jamos
pub fn decompose(c:char) -> Result<(char, char, Option<char>), HangeulError> {
    let cho = try!(get_choseong(c));
    let jung = try!(get_jungseong(c));
    let jong = try!(get_jongseong(c));
    Ok((cho, jung, jong))
}

/// Compose compatible jamos into a Hangeul syllable
pub fn compose(choseong:char, jungseong:char, jongseong:Option<char>) -> Result<char, HangeulError> {
    let cho_code = choseong as u32;
    let jung_code = jungseong as u32;
    let jong_code = match jongseong {
        Some(c) => c as u32,
        None => 0
    };

    if !_is_choseong(cho_code) || !_is_moeum(jung_code) || !(jong_code == 0 || _is_jongseong(jong_code)) {
        return Err(HangeulError::Uncomposable);
    }

    let composed = match jong_code == 0 {
        true => JAMO_TO_CHOSEONG[(cho_code - JAEUM_START) as usize] as u32 * 21 * 28
                + (jung_code - MOEUM_START) * 28
                + SYLLABLE_START,
        false => JAMO_TO_CHOSEONG[(cho_code - JAEUM_START) as usize] as u32 * 21 * 28
                + (jung_code - MOEUM_START) * 28
                + JAMO_TO_JONGSEONG[(jong_code - JAEUM_START) as usize] as u32 + 1
                + SYLLABLE_START
    };
    
    std::char::from_u32(composed).ok_or(HangeulError::Uncomposable)
}
