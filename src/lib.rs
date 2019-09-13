pub mod errors;
pub mod models;

use crate::models::*;
use crate::errors::*;

const HANGEUL_OFFSET: u32 = 0xAC00; // 44032

const SYLLABLE_START: u32 = 0xAC00;
const SYLLABLE_END: u32 = 0xD7A3;
const JAMO_START: u32 = 0x1100;
const JAMO_END: u32 = 0x11FF;
const COMPAT_JAMO_START: u32 = 0x3131;
const COMPAT_JAMO_END: u32 = 0x318E;

// lead/first chars
const CHOSEONG_START: u32 = 0x1100;
const CHOSEONG_END: u32 = 0x1112;
const COMPAT_CHOSEONG_START: u32 = 0x3131;
const COMPAT_CHOSEONG_END: u32 = 0x314E;

// middle chars / vowels
const JUNGSEONG_START: u32 = 0x1161;
const JUNGSEONG_END: u32 = 0x1175;
const COMPAT_JUNGSEONG_START: u32 = 0x314F;
const COMPAT_JUNGSEONG_END: u32 = 0x3163;

// tail/bottom/end chars
const JONGSEONG_START: u32 = 0x11A8;
const JONGSEONG_END: u32 = 0x11C2;
const COMPAT_JONGSEONG_START: u32 = 0x3165;
const COMPAT_JONGSEONG_END: u32 = 0x318E;

/// Check if the syllable is finished Hangeul syllable.
/// 0xAC00 to 0xD7A3
pub fn is_syllable(code: u32) -> bool {
    (code >= SYLLABLE_START && code <= SYLLABLE_END)
}

/// Checks if the character is a consonant, or jaeum (자음).
pub fn is_jaeum(code: u32) -> bool {
    (code >= COMPAT_CHOSEONG_START && code <= COMPAT_CHOSEONG_END)
    || (code >= COMPAT_JONGSEONG_START && code <= COMPAT_JONGSEONG_END)
    || (code >= CHOSEONG_START && code <= CHOSEONG_END)
    || (code >= JONGSEONG_START && code <= JONGSEONG_END)
}
/// Alias for is_jaeum.
pub use self::is_jaeum as is_consonant;

/// Checks if the character is a vowel, or moeum (모음).
pub fn is_moeum(code: u32) -> bool {
    (code >= COMPAT_JUNGSEONG_START && code <= COMPAT_JUNGSEONG_END)
    || (code >= JUNGSEONG_START && code <= JUNGSEONG_END)
}
/// Alias for is_moeum.
pub use self::is_moeum as is_vowel;

/// Returns true from 0x1100 to 0x11FF.
/// See https://en.wikipedia.org/wiki/Hangul_Jamo_(Unicode_block).
pub fn is_jamo(code: u32) -> bool {
    (code >= JAMO_START && code <= JAMO_END)
}

/// Returns true from 0x3131 to 0x318E.
/// See https://en.wikipedia.org/wiki/Hangul_Compatibility_Jamo.
pub fn is_compat_jamo(code: u32) -> bool {
    (code >= COMPAT_JAMO_START && code <= COMPAT_JAMO_END)
}

pub fn is_hangeul(code: u32) -> bool {
    if code >= SYLLABLE_START && code <= SYLLABLE_END {
        return true;
    } else if code >= JAMO_START && code <= JAMO_END {
        return true;
    } else if code >= COMPAT_JAMO_START && code <= COMPAT_JAMO_END {
        return true;
    }

    return false;
}

fn to_hangeul_u32(c: &char) -> Result<u32> {
    let code = *c as u32;

    match is_syllable(code) {
        true => Ok(code),
        false => Err(HangeulError::NotASyllable),
    }
}

/// Wrapper around `Choseong` to get the first/lead character from a Hangeul syllable.
/// See models::Choseong for more options for deconstruction.
pub fn get_choseong(c: &char) -> Result<char> {
    match Choseong::from_char(c) {
        Some(cho) => Ok(cho.to_char()),
        None => Err(HangeulError::JamoNotFound)
    }
}
/// Alias for get_choseong.
pub use self::get_choseong as get_lead;

/// Wrapper around `Jungseong` to get the middle/vowel character from a Hangeul syllable.
/// See models::Jungseong for more options for deconstruction.
pub fn get_jungseong(c: &char) -> Result<char> {
    match Jungseong::from_char(c) {
        Some(jung) => Ok(jung.to_char()),
        None => Err(HangeulError::JamoNotFound)
    }
}
/// Alias for get_jungseong.
pub use self::get_jungseong as get_middle;

/// Wrapper around `Jongseong` to get the bottom/tail character from a Hangeul syllable.
/// See models::Jongseong for more options for deconstruction.
pub fn get_jongseong(c: &char) -> Result<char> {
    match Jongseong::from_char(c) {
        Some(jong) => Ok(jong.to_char()),
        None => Err(HangeulError::JamoNotFound)
    }
}
/// Alias for get_jongseong.
pub use self::get_jongseong as get_tail;

/// Check if the syllable has a tail, or jongseong (종성).
pub fn has_jongseong(c: &char) -> Result<bool> {
    let code = to_hangeul_u32(c)?;

    Ok((code - HANGEUL_OFFSET) % 28 != 0)
}
/// Alias for has_jongseong.
pub use self::has_jongseong as has_tail;

pub fn is_choseong(code: u32) -> bool {
    Choseong::from_u32(code).is_some()
}
/// Alias for is_choseong.
pub use self::is_choseong as is_lead;

pub fn is_jungseong(code: u32) -> bool {
    (code >= COMPAT_JUNGSEONG_START && code <= COMPAT_JUNGSEONG_END)
    || (code >= JUNGSEONG_START && code <= JUNGSEONG_END)
}

pub fn is_jongseong(code: u32) -> bool {
    Jongseong::from_u32(code).is_some()
}
/// Alias for is_choseong.
pub use self::is_jongseong as is_tail;

type Decomposed = (char, char, Option<char>);

#[allow(dead_code)]
pub fn decompose(content: &str) -> Vec<Result<Decomposed>> {
    content
        .chars()
        .map(|c| decompose_char(&c))
        .collect()
}

#[allow(dead_code)]
pub fn decompose_char(c: &char) -> Result<Decomposed> {
    let choseong = get_choseong(c)?;
    let jungseong = get_jungseong(c)?;

    let jongseong = match Jongseong::from_char(c) {
        Some(jong) => Some(jong.to_char()),
        None => None,
    };

    Ok((
        choseong,
        jungseong,
        jongseong
    ))
}
