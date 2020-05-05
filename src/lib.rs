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

/// Check if the u32 is a finished/composed Hangeul syllable.
/// Returns true for the `0xAC00` to `0xD7A3` range.
///
/// ```rust
/// use hangeul::is_syllable;
///
/// assert_eq!(true, is_syllable('한' as u32));
/// assert_eq!(false, is_syllable('ㄱ' as u32));
/// assert_eq!(false, is_syllable('ㅏ' as u32));
/// ```
pub fn is_syllable(code: u32) -> bool {
    matches!(code, SYLLABLE_START..=SYLLABLE_END)
}

/// Checks if the u32 is a consonant, or jaeum (자음).
/// Checks both Jamo and Compatibility Jamo unicode blocks.
pub fn is_jaeum(code: u32) -> bool {
    matches!(
        code,
        COMPAT_CHOSEONG_START..=COMPAT_CHOSEONG_END
        | COMPAT_JONGSEONG_START..=COMPAT_JONGSEONG_END
        | CHOSEONG_START..=CHOSEONG_END
        | JONGSEONG_START..=JONGSEONG_END
    )
}
/// Alias for is_jaeum.
pub use self::is_jaeum as is_consonant;

/// Checks if the u32 is a vowel, or moeum (모음).
/// Checks both Jamo and Compatibility Jamo unicode blocks.
///
/// ```rust
/// use hangeul::is_moeum; // is_vowel
///
/// assert_eq!(true, is_moeum('ᅡ' as u32));
/// assert_eq!(true, is_moeum('ㅏ' as u32));
/// assert_eq!(false, is_moeum('ㄱ' as u32));
/// ```
pub fn is_moeum(code: u32) -> bool {
    matches!(code, COMPAT_JUNGSEONG_START..=COMPAT_JUNGSEONG_END | JUNGSEONG_START..=JUNGSEONG_END)
}
/// Alias for is_moeum.
pub use self::is_moeum as is_vowel;

/// Returns true from `0x1100` to `0x11FF`.
/// See [Hangeul Jamo](https://en.wikipedia.org/wiki/Hangul_Jamo_(Unicode_block)).
///
/// ```rust
/// use hangeul::is_jamo;
///
/// assert_eq!(true, is_jamo('ᅡ' as u32)); // 0x1161
/// assert_eq!(false, is_jamo('ㅏ' as u32)); // 0x314F
/// assert_eq!(false, is_jamo('한' as u32));
/// ```
pub fn is_jamo(code: u32) -> bool {
    matches!(code, JAMO_START..=JAMO_END)
}

/// Returns true from `0x3131` to `0x318E`.
/// See [Compatibility Jamo](https://en.wikipedia.org/wiki/Hangul_Compatibility_Jamo).
pub fn is_compat_jamo(code: u32) -> bool {
    matches!(code, COMPAT_JAMO_START..=COMPAT_JAMO_END)
}

/// Checks if a u32 is a (composable) Hangeul Syllable or Jamo.
/// Archaic Korean is not supported at this time.
/// See:
/// * https://en.wikipedia.org/wiki/Hangul_Syllables.
/// * https://en.wikipedia.org/wiki/Hangul_Jamo_(Unicode_block)
/// * https://en.wikipedia.org/wiki/Hangul_Compatibility_Jamo
///
/// ```rust
/// use hangeul::is_hangeul;
///
/// assert_eq!(true, is_hangeul('한' as u32));
/// assert_eq!(true, is_hangeul('ㅏ' as u32));
/// assert_eq!(true, is_hangeul('ᆶ' as u32));
/// assert_eq!(false, is_hangeul('a' as u32));
/// ```
pub fn is_hangeul(code: u32) -> bool {
    is_syllable(code) || is_jamo(code) || is_compat_jamo(code)
}

/// Casts a char to u32. Errors if said u32 falls outside of the Hangeul Syllable unicode range.
/// See `is_hangeul`.
///
/// ```rust
/// use hangeul::to_hangeul_u32;
/// use hangeul::errors::HangeulError;
///
/// assert_eq!(Ok(0xB242), to_hangeul_u32(&'뉂'));
/// assert_eq!(Ok(0xBDC6), to_hangeul_u32(&'뷆'));
/// assert_eq!(Err(HangeulError::NotASyllable), to_hangeul_u32(&'ㅏ'));
/// assert_eq!(Err(HangeulError::NotASyllable), to_hangeul_u32(&'何'));
/// ```
pub fn to_hangeul_u32(c: &char) -> Result<u32> {
    let code = *c as u32;

    match is_syllable(code) {
        true => Ok(code),
        false => Err(HangeulError::NotASyllable),
    }
}

/// Wrapper around `Choseong` to get the first/lead character from a Hangeul syllable.
/// See models::Choseong.
///
/// ```rust
/// use hangeul::get_choseong; // get_lead
/// use hangeul::errors::HangeulError;
/// use hangeul::models::Choseong;
///
/// assert_eq!(Ok('ㄱ'), get_choseong(&'갅'));
/// assert_eq!(Err(HangeulError::JamoNotFound), get_choseong(&'ㅏ'));
/// ```
pub fn get_choseong(c: &char) -> Result<char> {
    match Choseong::from_char(c) {
        Some(cho) => Ok(cho.to_char()),
        None => Err(HangeulError::JamoNotFound)
    }
}
/// Alias for get_choseong.
pub use self::get_choseong as get_lead;

/// Wrapper around `Jungseong` to get the middle/vowel character from a Hangeul syllable.
/// See models::Jungseong.
///
/// ```rust
/// use hangeul::get_jungseong; // get_middle
/// use hangeul::errors::HangeulError;
/// use hangeul::models::Jungseong;
///
/// assert_eq!(Ok('ㅏ'), get_jungseong(&'갅'));
/// assert_eq!(Err(HangeulError::JamoNotFound), get_jungseong(&'ㄱ'));
/// ```
pub fn get_jungseong(c: &char) -> Result<char> {
    match Jungseong::from_char(c) {
        Some(jung) => Ok(jung.to_char()),
        None => Err(HangeulError::JamoNotFound)
    }
}
/// Alias for get_jungseong.
pub use self::get_jungseong as get_middle;

/// Wrapper around `Jongseong` to get the bottom/tail character from a Hangeul syllable.
/// See models::Jongseong.
///
/// ```rust
/// use hangeul::get_jongseong; // get_tail
/// use hangeul::errors::HangeulError;
/// use hangeul::models::Jongseong;
///
/// assert_eq!(Ok('ㄵ'), get_jongseong(&'갅'));
/// assert_eq!(Err(HangeulError::JamoNotFound), get_jongseong(&'ㅏ'));
/// ```
pub fn get_jongseong(c: &char) -> Result<char> {
    match Jongseong::from_char(c) {
        Some(jong) => Ok(jong.to_char()),
        None => Err(HangeulError::JamoNotFound)
    }
}
/// Alias for get_jongseong.
pub use self::get_jongseong as get_tail;

/// Check if the syllable has a tail, or jongseong (종성).
/// use hangeul::has_jongseong; // has_jongseong
/// ```rust
/// use hangeul::has_jongseong;
/// use hangeul::errors::HangeulError;
///
/// assert_eq!(Ok(true), has_jongseong(&'갅'));
/// assert_eq!(Ok(false), has_jongseong(&'가'));
/// assert_eq!(Err(HangeulError::NotASyllable), has_jongseong(&'b'));
/// ```
pub fn has_jongseong(c: &char) -> Result<bool> {
    let code = to_hangeul_u32(c)?;

    Ok((code - HANGEUL_OFFSET) % 28 != 0)
}
/// Alias for has_jongseong.
pub use self::has_jongseong as has_tail;

/// Checks if a char is a lead jamo, or choseong (중성).
///
/// ```rust
/// use hangeul::is_choseong; //is_lead
///
/// assert_eq!(true, is_choseong('ㅉ' as u32));
/// assert_eq!(false, is_choseong('ㅏ' as u32));
/// assert_eq!(false, is_choseong('ㄵ' as u32));
/// ```
pub fn is_choseong(code: u32) -> bool {
    Choseong::from_u32(code).is_some()
}
/// Alias for is_choseong.
pub use self::is_choseong as is_lead;

/// Checks if a char is a middle/vowel jamo, or jungseong (중성).
///
/// ```rust
/// use hangeul::is_jungseong; //is_vowel
///
/// assert_eq!(false, is_jungseong('ㅉ' as u32));
/// assert_eq!(true, is_jungseong('ㅏ' as u32));
/// assert_eq!(false, is_jungseong('ㄵ' as u32));
/// ```
pub fn is_jungseong(code: u32) -> bool {
    Jungseong::from_u32(code).is_some()
}

/// Checks if a char is a tail jamo, or jongseong (종성).
///
/// ```rust
/// use hangeul::is_jongseong; //is_tail
///
/// assert_eq!(false, is_jongseong('ㅉ' as u32));
/// assert_eq!(false, is_jongseong('ㅏ' as u32));
/// assert_eq!(true, is_jongseong('ㄵ' as u32));
/// ```
pub fn is_jongseong(code: u32) -> bool {
    Jongseong::from_u32(code).is_some()
}
/// Alias for is_choseong.
pub use self::is_jongseong as is_tail;

/// Checks if a str's last char ends in a consonant or not.
///
/// ```rust
/// use hangeul::ends_with_jongseong; // ends_with_consonant
///
/// assert_eq!(Ok(false), ends_with_jongseong("피카츄"));
/// assert_eq!(Ok(true), ends_with_jongseong("이상해꽃"));
/// ```
pub fn ends_with_jongseong(content: &str) -> Result<bool> {
    match content.chars().last() {
        Some(x) => has_jongseong(&x),
        None => Err(HangeulError::NotASyllable),
    }
}
/// Alias for ends_with_jongseong.
pub use self::ends_with_jongseong as ends_in_consonant;

type Decomposed = (char, char, Option<char>);

/// Attempts to decompose a string of Hangeul characters. See `decompose_char`.
///
/// ```rust
/// use hangeul::decompose;
///
/// let dae_han = "대한";
/// let decomposed = vec![
///     Ok(('ㄷ', 'ㅐ', None)),
///     Ok(('ㅎ', 'ㅏ', Some('ㄴ'))),
/// ];
/// assert_eq!(decomposed, decompose(dae_han));
/// ```
pub fn decompose(content: &str) -> Vec<Result<Decomposed>> {
    content
        .chars()
        .map(|c| decompose_char(&c))
        .collect()
}

/// Attempts to decompose a char. Errors if the first and second glyphs
/// aren't valid Korean jamo. See [Compatibility Jamo](https://en.wikipedia.org/wiki/Hangul_Compatibility_Jamo).
///
/// ```rust
/// use hangeul::decompose_char;
///
/// let bap = '밮';
/// let b_a_p = ('ㅂ', 'ㅏ', Some('ㅍ'));
/// assert_eq!(b_a_p, decompose_char(&bap).unwrap());
/// ```
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


/// Attempts to compose a Hangeul character (in the `Hangeul Syllable` unicode range,
/// `AC00`–`D7A3`). See [Hangeul Syllables](https://en.wikipedia.org/wiki/Hangul_Syllables).
///
/// ```rust
/// use hangeul::compose_char;
///
/// let bap = '밮';
/// let b = 'ㅂ';
/// let a = 'ㅏ';
/// let p = 'ㅍ';
/// assert_eq!(bap, compose_char(&b, &a, Some(&p)).unwrap());
/// ```
pub fn compose_char(choseong: &char, jungseong: &char, jongseong: Option<&char>) -> Result<char> {
    let cho_code = match Choseong::from_char(choseong) {
        Some(cho) => cho.composable_u32(),
        None => return Err(HangeulError::Uncomposable)
    };

    let jung_code = match Jungseong::from_char(jungseong) {
        Some(jung) => jung.composable_u32(),
        None => return Err(HangeulError::Uncomposable)
    };

    let jong_code = match jongseong {
        Some(c) => {
            match Jongseong::from_char(c) {
                Some(jong) => jong.composable_u32(),
                None => 0,
            }
        },
        None => 0
    };

    let code = cho_code + jung_code + jong_code + HANGEUL_OFFSET;

    std::char::from_u32(code)
        .ok_or_else(|| HangeulError::Uncomposable)
}
