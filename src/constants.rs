pub const HANGEUL_OFFSET: u32 = 0xAC00; // 44032

pub const SYLLABLE_START: u32 = 0xAC00;
pub const SYLLABLE_END: u32 = 0xD7A3;

pub const CHOSEONG_COUNT: u32 = 588;
pub const JUNGSEONG_COUNT: u32 = 28;

pub const JAMO_START: u32 = 0x1100;
pub const JAMO_END: u32 = 0x11FF;
pub const COMPATIBLE_JAMO_START: u32 = 0x3131;
pub const COMPATIBLE_JAMO_END: u32 = 0x318E;

// lead/first chars
pub const CHOSEONG_START: u32 = 0x1100;
pub const CHOSEONG_END: u32 = 0x1112;
pub const COMPAT_CHOSEONG_START: u32 = 0x3131;
pub const COMPAT_CHOSEONG_END: u32 = 0x314E;

// middle chars / vowels
pub const JUNGSEONG_START: u32 = 0x1161;
pub const JUNGSEONG_END: u32 = 0x1175;
pub const COMPAT_JUNGSEONG_START: u32 = 0x314F;
pub const COMPAT_JUNGSEONG_END: u32 = 0x3163;

// tail/bottom/end chars
pub const JONGSEONG_START: u32 = 0x11A8;
pub const JONGSEONG_END: u32 = 0x11C2;
pub const COMPAT_JONGSEONG_START: u32 = 0x3165;
pub const COMPAT_JONGSEONG_END: u32 = 0x318E;
