extern crate hangeul;
use hangeul::*;

#[test]
fn check_decompose() {
    assert_eq!(
        vec![
            Ok(('ㄷ', 'ㅐ', None)),
            Ok(('ㅎ', 'ㅏ', Some('ㄴ'))),
            Ok(('ㅁ', 'ㅣ', Some('ㄴ'))),
            Ok(('ㄱ', 'ㅜ', Some('ㄱ'))),
        ],
        decompose("대한민국")
    );
}

#[test]
fn check_decompose_char() {
    assert_eq!(Ok(('ㅎ', 'ㅏ', Some('ㄴ'))), decompose_char(&'한'));

    assert_eq!(Ok(('ㅎ', 'ㅏ', None)), decompose_char(&'하'));
}

#[test]
fn check_jamo() {
    assert_eq!(is_jamo('\u{1100}' as u32), true);
    assert_eq!(is_compat_jamo('ㄱ' as u32), true);
    assert_eq!(is_jamo('\u{1175}' as u32), true);
    assert_eq!(is_compat_jamo('ㅣ' as u32), true);
    assert_eq!(is_jamo('a' as u32), false);
}

#[test]
fn check_jaeum() {
    assert_eq!(is_jaeum('ㄱ' as u32), true);
    assert_eq!(is_jaeum('ㅎ' as u32), true);
    assert_eq!(is_jaeum('ㅏ' as u32), false);
}

#[test]
fn check_choseong() {
    assert_eq!(is_choseong('ㄱ' as u32), true);
    assert_eq!(is_choseong('ㅎ' as u32), true);
    assert_eq!(is_choseong('ㄸ' as u32), true);
    assert_eq!(is_choseong('ㄳ' as u32), false);
    assert_eq!(is_choseong('ㅉ' as u32), true);
    assert_eq!(is_choseong('ㅃ' as u32), true);
    assert_eq!(is_choseong('ㅄ' as u32), false);
    assert_eq!(is_choseong('\u{3130}' as u32), false);
    assert_eq!(is_choseong('\u{314F}' as u32), false);
}

#[test]
fn check_jungseong() {
    assert_eq!(is_jungseong('ㅏ' as u32), true);
    assert_eq!(is_jungseong('ㅗ' as u32), true);
    assert_eq!(is_jungseong('ㅂ' as u32), false);
    assert_eq!(is_jungseong('z' as u32), false);
    assert_eq!(is_jungseong('ㄳ' as u32), false);
}

#[test]
fn check_jongseong() {
    assert_eq!(is_jongseong('ㄱ' as u32), true);
    assert_eq!(is_jongseong('ㅎ' as u32), true);
    assert_eq!(is_jongseong('ㄸ' as u32), false);
    assert_eq!(is_jongseong('ㄳ' as u32), true);
    assert_eq!(is_jongseong('ㅉ' as u32), false);
    assert_eq!(is_jongseong('ㅃ' as u32), false);
    assert_eq!(is_jongseong('ㅄ' as u32), true);
    assert_eq!(is_jongseong('A' as u32), false);
    assert_eq!(is_jongseong('\u{3130}' as u32), false);
    assert_eq!(is_jongseong('\u{314F}' as u32), false);
}

#[test]
fn check_vowel_consonant() {
    assert_eq!(is_consonant('ㄱ' as u32), true);
    assert_eq!(is_consonant('ㅐ' as u32), false);
    assert_eq!(is_consonant('가' as u32), false);
    assert_eq!(is_vowel('ㅏ' as u32), true);
    assert_eq!(is_vowel('ㅁ' as u32), false);
    assert_eq!(is_vowel('오' as u32), false);
}

#[test]
fn check_position() {
    assert_eq!(is_jongseong('ㄳ' as u32), true);
    assert_eq!(has_jongseong(&'감'), Ok(true));

    let gam = '감';
    assert_eq!(get_lead(&gam).unwrap(), 'ㄱ');
    assert_eq!(get_middle(&gam).unwrap(), 'ㅏ');
    assert_eq!(get_tail(&gam).unwrap(), 'ㅁ');
}
