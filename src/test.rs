use super::*;
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

#[test]
fn decomposition_test() {
    let han = '한';
    let ha = '하';
    assert_eq!(get_choseong(han).unwrap(), 'ㅎ');
    assert_eq!(get_jungseong(han).unwrap(), 'ㅏ');
    assert_eq!(get_jongseong(han).unwrap().unwrap(), 'ㄴ');
    assert_eq!(has_jongseong(han).unwrap(), true);
    assert_eq!(has_jongseong(ha).unwrap(), false);
    assert_eq!(get_jongseong(ha).unwrap(), None);
}

#[test]
fn check_jamo_test() {
    assert_eq!(is_jamo('ㄱ'), true);
    assert_eq!(is_jamo('ㅣ'), true);
    assert_eq!(is_jamo('a'), false);
    assert_eq!(is_jaeum('ㄱ'), true);
    assert_eq!(is_jaeum('ㅎ'), true);
    assert_eq!(is_jaeum('ㅏ'), false);
    assert_eq!(is_choseong('ㄱ'), true);
    assert_eq!(is_choseong('ㅎ'), true);
    assert_eq!(is_choseong('ㄸ'), true);
    assert_eq!(is_choseong('ㄳ'), false);
    assert_eq!(is_choseong('ㅉ'), true);
    assert_eq!(is_choseong('ㅃ'), true);
    assert_eq!(is_choseong('ㅄ'), false);
    assert_eq!(is_choseong('\u{3130}'), false);
    assert_eq!(is_choseong('\u{314F}'), false);
    assert_eq!(is_jongseong('ㄱ'), true);
    assert_eq!(is_jongseong('ㅎ'), true);
    assert_eq!(is_jongseong('ㄸ'), false);
    assert_eq!(is_jongseong('ㄳ'), true);
    assert_eq!(is_jongseong('ㅉ'), false);
    assert_eq!(is_jongseong('ㅃ'), false);
    assert_eq!(is_jongseong('ㅄ'), true);
    assert_eq!(is_jongseong('A'), false);
    assert_eq!(is_jongseong('\u{3130}'), false);
    assert_eq!(is_jongseong('\u{314F}'), false);
}

#[test]
fn compose_test() {
    assert_eq!(compose('ㄱ', 'ㅏ', None).unwrap(), '가');
    assert_eq!(compose('ㄱ', 'ㅏ', Some('ㄱ')).unwrap(), '각');
    assert_eq!(compose('ㄱ', 'ㅏ', Some('ㅄ')).unwrap(), '값');
    assert_eq!(compose('ㅎ', 'ㅘ', Some('ㅎ')).unwrap(), '홯');
    compose('ㄳ', 'ㅏ', None).unwrap_err();
}

#[test]
fn alias_test() {
    assert_eq!(is_top('ㄱ'), true);
    assert_eq!(is_bottom('ㄳ'), true);
    assert_eq!(has_bottom('감').unwrap(), true);
    assert_eq!(ends_with_bottom(&"감감").unwrap(), true);
    assert_eq!(is_consonant('ㄱ'), true);
    assert_eq!(is_vowel('ㅏ'), true);
    let gam = '감';
    assert_eq!(get_top(gam).unwrap(), 'ㄱ');
    assert_eq!(get_middle(gam).unwrap(), 'ㅏ');
    assert_eq!(get_bottom(gam).unwrap().unwrap(), 'ㅁ');
}
