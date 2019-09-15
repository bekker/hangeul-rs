use crate::is_syllable;

const HANGEUL_OFFSET: u32 = 0xAC00; // 44032

pub trait Syllable {
    fn composable_u32(&self) -> u32;
    fn to_u32(&self) -> u32;
}

#[derive(Debug, PartialEq)]
pub enum Choseong {
    Giyeok,
    SsangGiyeok,
    Nieun,
    Digeut,
    SsangDigeut,
    Rieul,
    Mieum,
    Bieup,
    SsangBieup,
    Siot,
    SsangSiot,
    Ieung,
    Jieut,
    SsangJieut,
    Chieut,
    Kiyeok,
    Tieut,
    Pieup,
    Hieuh,
}

/// `초성` -> Initial Sound, aka Initial or Lead Character
impl Choseong {
    pub fn from_char(c: &char) -> Option<Choseong> {
        Choseong::from_u32(*c as u32)
    }

    pub fn from_u32(code: u32) -> Option<Choseong> {
        match is_syllable(code) {
            true => Choseong::from_syllable(code),
            false => Choseong::from_jamo(code),
        }
    }

    pub fn from_syllable(code: u32) -> Option<Choseong> {
        let value = 1 + (code - HANGEUL_OFFSET) / 588;

        match value {
            1  => Some(Choseong::Giyeok),
            2  => Some(Choseong::SsangGiyeok),
            3  => Some(Choseong::Nieun),
            4  => Some(Choseong::Digeut),
            5  => Some(Choseong::SsangDigeut),
            6  => Some(Choseong::Rieul),
            7  => Some(Choseong::Mieum),
            8  => Some(Choseong::Bieup),
            9  => Some(Choseong::SsangBieup),
            10 => Some(Choseong::Siot),
            11 => Some(Choseong::SsangSiot),
            12 => Some(Choseong::Ieung),
            13 => Some(Choseong::Jieut),
            14 => Some(Choseong::SsangJieut),
            15 => Some(Choseong::Chieut),
            16 => Some(Choseong::Kiyeok),
            17 => Some(Choseong::Tieut),
            18 => Some(Choseong::Pieup),
            19 => Some(Choseong::Hieuh),
            _ => None,
        }
    }

    pub fn from_jamo(code: u32) -> Option<Choseong> {
        match code {
             0x1100 => Some(Choseong::Giyeok),
             0x3131 => Some(Choseong::Giyeok),
             0x1101 => Some(Choseong::SsangGiyeok),
             0x3132 => Some(Choseong::SsangGiyeok),
             0x1102 => Some(Choseong::Nieun),
             0x3134 => Some(Choseong::Nieun),
             0x1103 => Some(Choseong::Digeut),
             0x3137 => Some(Choseong::Digeut),
             0x1104 => Some(Choseong::SsangDigeut),
             0x3138 => Some(Choseong::SsangDigeut),
             0x1105 => Some(Choseong::Rieul),
             0x3139 => Some(Choseong::Rieul),
             0x1106 => Some(Choseong::Mieum),
             0x3141 => Some(Choseong::Mieum),
             0x1107 => Some(Choseong::Bieup),
             0x3142 => Some(Choseong::Bieup),
             0x1108 => Some(Choseong::SsangBieup),
             0x3143 => Some(Choseong::SsangBieup),
             0x1109 => Some(Choseong::Siot),
             0x3145 => Some(Choseong::Siot),
             0x110A => Some(Choseong::SsangSiot),
             0x3146 => Some(Choseong::SsangSiot),
             0x110B => Some(Choseong::Ieung),
             0x3147 => Some(Choseong::Ieung),
             0x110C => Some(Choseong::Jieut),
             0x3148 => Some(Choseong::Jieut),
             0x110D => Some(Choseong::SsangJieut),
             0x3149 => Some(Choseong::SsangJieut),
             0x110E => Some(Choseong::Chieut),
             0x313A => Some(Choseong::Chieut),
             0x110F => Some(Choseong::Kiyeok),
             0x314B => Some(Choseong::Kiyeok),
             0x1110 => Some(Choseong::Tieut),
             0x314C => Some(Choseong::Tieut),
             0x1111 => Some(Choseong::Pieup),
             0x314D => Some(Choseong::Pieup),
             0x1112 => Some(Choseong::Hieuh),
             0x314E => Some(Choseong::Hieuh),
             _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            Choseong::Giyeok      => 'ㄱ',
            Choseong::SsangGiyeok => 'ㄲ',
            Choseong::Nieun       => 'ㄴ',
            Choseong::Digeut      => 'ㄷ',
            Choseong::SsangDigeut => 'ㄸ',
            Choseong::Rieul       => 'ㄹ',
            Choseong::Mieum       => 'ㅁ',
            Choseong::Bieup       => 'ㅂ',
            Choseong::SsangBieup  => 'ㅃ',
            Choseong::Siot        => 'ㅅ',
            Choseong::SsangSiot   => 'ㅆ',
            Choseong::Ieung       => 'ㅇ',
            Choseong::Jieut       => 'ㅈ',
            Choseong::SsangJieut  => 'ㅉ',
            Choseong::Chieut      => 'ㅊ',
            Choseong::Kiyeok      => 'ㅋ',
            Choseong::Tieut       => 'ㅌ',
            Choseong::Pieup       => 'ㅍ',
            Choseong::Hieuh       => 'ㅎ',
        }
    }
}

impl Syllable for Choseong {
    fn to_u32(&self) -> u32 {
        match *self {
            Choseong::Giyeok      => 0x1100,
            Choseong::SsangGiyeok => 0x1101,
            Choseong::Nieun       => 0x1102,
            Choseong::Digeut      => 0x1103,
            Choseong::SsangDigeut => 0x1104,
            Choseong::Rieul       => 0x1105,
            Choseong::Mieum       => 0x1106,
            Choseong::Bieup       => 0x1107,
            Choseong::SsangBieup  => 0x1108,
            Choseong::Siot        => 0x1109,
            Choseong::SsangSiot   => 0x110A,
            Choseong::Ieung       => 0x110B,
            Choseong::Jieut       => 0x110C,
            Choseong::SsangJieut  => 0x110D,
            Choseong::Chieut      => 0x110E,
            Choseong::Kiyeok      => 0x110F,
            Choseong::Tieut       => 0x1110,
            Choseong::Pieup       => 0x1111,
            Choseong::Hieuh       => 0x1112,
        }
    }

    fn composable_u32(&self) -> u32 {
        let value = match self {
            Choseong::Giyeok      => 0,
            Choseong::SsangGiyeok => 1,
            Choseong::Nieun       => 2,
            Choseong::Digeut      => 3,
            Choseong::SsangDigeut => 4,
            Choseong::Rieul       => 5,
            Choseong::Mieum       => 6,
            Choseong::Bieup       => 7,
            Choseong::SsangBieup  => 8,
            Choseong::Siot        => 9,
            Choseong::SsangSiot   => 10,
            Choseong::Ieung       => 11,
            Choseong::Jieut       => 12,
            Choseong::SsangJieut  => 13,
            Choseong::Chieut      => 14,
            Choseong::Kiyeok      => 15,
            Choseong::Tieut       => 16,
            Choseong::Pieup       => 17,
            Choseong::Hieuh       => 18,
        };

        value * 588
    }
}

/// `중성` -> Middle Sound (always a vowel)
#[derive(Debug, PartialEq)]
pub enum Jungseong {
    A,   // ㅏ
    AE,  // ㅐ
    YA,  // ㅑ
    YAE, // ㅒ
    EO,  // ㅓ
    E,   // ㅔ
    YEO, // ㅕ
    YE,  // ㅖ
    O,   // ㅗ
    WA,  // ㅘ
    WAE, // ㅙ
    OE,  // ㅚ
    YO,  // ㅛ
    U,   // ㅜ
    WEO, // ㅝ
    WE,  // ㅞ
    WI,  // ㅢ
    YU,  // ㅠ
    EU,  // ㅡ
    YI,  // 의
    I,   // ㅣ
}

impl Jungseong {
    pub fn from_char(c: &char) -> Option<Jungseong> {
        Jungseong::from_u32(*c as u32)
    }

    pub fn from_u32(code: u32) -> Option<Jungseong> {
        match is_syllable(code) {
            true => Jungseong::from_syllable(code),
            false => Jungseong::from_jamo(code),
        }
    }

    pub fn from_syllable(code: u32) -> Option<Jungseong> {
        let jongseong_code = (code - HANGEUL_OFFSET) % 28;
        let value = 1 + (((code - HANGEUL_OFFSET - jongseong_code) % 588) / 28);

        match value {
            1  => Some(Jungseong::A),
            2  => Some(Jungseong::AE),
            3  => Some(Jungseong::YA),
            4  => Some(Jungseong::YAE),
            5  => Some(Jungseong::EO),
            6  => Some(Jungseong::E),
            7  => Some(Jungseong::YEO),
            8  => Some(Jungseong::YE),
            9  => Some(Jungseong::O),
            10 => Some(Jungseong::WA),
            11 => Some(Jungseong::WAE),
            12 => Some(Jungseong::OE),
            13 => Some(Jungseong::YO),
            14 => Some(Jungseong::U),
            15 => Some(Jungseong::WEO),
            16 => Some(Jungseong::WE),
            17 => Some(Jungseong::WI),
            18 => Some(Jungseong::YU),
            19 => Some(Jungseong::EU),
            20 => Some(Jungseong::YI),
            21 => Some(Jungseong::I),
            _ => None,
        }
    }

    pub fn from_jamo(code: u32) -> Option<Jungseong> {
        match code {
             0x1161 => Some(Jungseong::A),
             0x314F => Some(Jungseong::A),
             0x1162 => Some(Jungseong::AE),
             0x3150 => Some(Jungseong::AE),
             0x1163 => Some(Jungseong::YA),
             0x3151 => Some(Jungseong::YA),
             0x1164 => Some(Jungseong::YAE),
             0x3152 => Some(Jungseong::YAE),
             0x1165 => Some(Jungseong::EO),
             0x3153 => Some(Jungseong::EO),
             0x1166 => Some(Jungseong::E),
             0x3154 => Some(Jungseong::E),
             0x1167 => Some(Jungseong::YEO),
             0x3155 => Some(Jungseong::YEO),
             0x1168 => Some(Jungseong::YE),
             0x3156 => Some(Jungseong::YE),
             0x1169 => Some(Jungseong::O),
             0x3157 => Some(Jungseong::O),
             0x116A => Some(Jungseong::WA),
             0x3158 => Some(Jungseong::WA),
             0x116B => Some(Jungseong::WAE),
             0x3159 => Some(Jungseong::WAE),
             0x116C => Some(Jungseong::OE),
             0x315A => Some(Jungseong::OE),
             0x116D => Some(Jungseong::YO),
             0x315B => Some(Jungseong::YO),
             0x116E => Some(Jungseong::U),
             0x315C => Some(Jungseong::U),
             0x116F => Some(Jungseong::WEO),
             0x315D => Some(Jungseong::WEO),
             0x1170 => Some(Jungseong::WE),
             0x315E => Some(Jungseong::WE),
             0x1171 => Some(Jungseong::WI),
             0x315F => Some(Jungseong::WI),
             0x1172 => Some(Jungseong::YU),
             0x3160 => Some(Jungseong::YU),
             0x1173 => Some(Jungseong::EU),
             0x3161 => Some(Jungseong::EU),
             0x1174 => Some(Jungseong::YI),
             0x3162 => Some(Jungseong::YI),
             0x1175 => Some(Jungseong::I),
             0x3163 => Some(Jungseong::I),
             _ => None,
        }
    }


    pub fn to_char(&self) -> char {
        match *self {
            Jungseong::A   => 'ㅏ',
            Jungseong::AE  => 'ㅐ',
            Jungseong::YA  => 'ㅑ',
            Jungseong::YAE => 'ㅒ',
            Jungseong::EO  => 'ㅓ',
            Jungseong::E   => 'ㅔ',
            Jungseong::YEO => 'ㅕ',
            Jungseong::YE  => 'ㅖ',
            Jungseong::O   => 'ㅗ',
            Jungseong::WA  => 'ㅘ',
            Jungseong::WAE => 'ㅙ',
            Jungseong::OE  => 'ㅚ',
            Jungseong::YO  => 'ㅛ',
            Jungseong::U   => 'ㅜ',
            Jungseong::WEO => 'ㅝ',
            Jungseong::WE  => 'ㅞ',
            Jungseong::WI  => 'ㅟ',
            Jungseong::YU  => 'ㅠ',
            Jungseong::EU  => 'ㅡ',
            Jungseong::YI  => 'ㅢ',
            Jungseong::I   => 'ㅣ',
        }
    }
}

impl Syllable for Jungseong {
    fn to_u32(&self) -> u32 {
        match *self {
            Jungseong::A   => 0x1161,
            Jungseong::AE  => 0x1162,
            Jungseong::YA  => 0x1163,
            Jungseong::YAE => 0x1164,
            Jungseong::EO  => 0x1165,
            Jungseong::E   => 0x1166,
            Jungseong::YEO => 0x1167,
            Jungseong::YE  => 0x1168,
            Jungseong::O   => 0x1169,
            Jungseong::WA  => 0x116A,
            Jungseong::WAE => 0x116B,
            Jungseong::OE  => 0x116C,
            Jungseong::YO  => 0x116D,
            Jungseong::U   => 0x116E,
            Jungseong::WEO => 0x116F,
            Jungseong::WE  => 0x1170,
            Jungseong::WI  => 0x1171,
            Jungseong::YU  => 0x1172,
            Jungseong::EU  => 0x1173,
            Jungseong::YI  => 0x1174,
            Jungseong::I   => 0x1175,
        }
    }

    fn composable_u32(&self) -> u32 {
        let value = match *self {
            Jungseong::A   => 0,
            Jungseong::AE  => 1,
            Jungseong::YA  => 2,
            Jungseong::YAE => 3,
            Jungseong::EO  => 4,
            Jungseong::E   => 5,
            Jungseong::YEO => 6,
            Jungseong::YE  => 7,
            Jungseong::O   => 8,
            Jungseong::WA  => 9,
            Jungseong::WAE => 10,
            Jungseong::OE  => 11,
            Jungseong::YO  => 12,
            Jungseong::U   => 13,
            Jungseong::WEO => 14,
            Jungseong::WE  => 15,
            Jungseong::WI  => 16,
            Jungseong::YU  => 17,
            Jungseong::EU  => 18,
            Jungseong::YI  => 19,
            Jungseong::I   => 20,
        };

        value * 28
    }
}

#[derive(Debug, PartialEq)]
pub enum Jongseong {
    Giyeok,
    SsangGiyeok,
    GiyeokSiot,
    Nieun,
    NieunJieut,
    NieunHieuh,
    Digeut,
    Rieul,
    RieulGiyeok,
    RieulMieum,
    RieulBieup,
    RieulSiot,
    RieulTieut,
    RieulPieup,
    RieulHieuh,
    Mieum,
    Bieup,
    BieupSiot,
    Siot,
    SsangSiot,
    Ieung,
    Jieut,
    Chieut,
    Kieuk,
    Tieut,
    Pieup,
    Hieuh,
}

impl Jongseong {
    pub fn from_char(c: &char) -> Option<Jongseong> {
        Jongseong::from_u32(*c as u32)
    }

    pub fn from_u32(code: u32) -> Option<Jongseong> {
        match is_syllable(code) {
            true => Jongseong::from_syllable(code),
            false => Jongseong::from_jamo(code),
        }
    }

    pub fn from_syllable(code: u32) -> Option<Jongseong> {
        let value = (code - HANGEUL_OFFSET) % 28;

        match value {
            1  => Some(Jongseong::Giyeok),
            2  => Some(Jongseong::SsangGiyeok),
            3  => Some(Jongseong::GiyeokSiot),
            4  => Some(Jongseong::Nieun),
            5  => Some(Jongseong::NieunJieut),
            6  => Some(Jongseong::NieunHieuh),
            7  => Some(Jongseong::Digeut),
            8  => Some(Jongseong::Rieul),
            9  => Some(Jongseong::RieulGiyeok),
            10 => Some(Jongseong::RieulMieum),
            11 => Some(Jongseong::RieulBieup),
            12 => Some(Jongseong::RieulSiot),
            13 => Some(Jongseong::RieulTieut),
            14 => Some(Jongseong::RieulPieup),
            15 => Some(Jongseong::RieulHieuh),
            16 => Some(Jongseong::Mieum),
            17 => Some(Jongseong::Bieup),
            18 => Some(Jongseong::BieupSiot),
            19 => Some(Jongseong::Siot),
            20 => Some(Jongseong::SsangSiot),
            21 => Some(Jongseong::Ieung),
            22 => Some(Jongseong::Jieut),
            23 => Some(Jongseong::Chieut),
            24 => Some(Jongseong::Kieuk),
            25 => Some(Jongseong::Tieut),
            26 => Some(Jongseong::Pieup),
            27 => Some(Jongseong::Hieuh),
            _ => None,
        }
    }

    pub fn from_jamo(code: u32) -> Option<Jongseong> {
        match code {
             0x11A8 => Some(Jongseong::Giyeok),
             0x3131 => Some(Jongseong::Giyeok),
             0x11A9 => Some(Jongseong::SsangGiyeok),
             0x3132 => Some(Jongseong::SsangGiyeok),
             0x11AA => Some(Jongseong::GiyeokSiot),
             0x3133 => Some(Jongseong::GiyeokSiot),
             0x11AB => Some(Jongseong::Nieun),
             0x3134 => Some(Jongseong::Nieun),
             0x11AC => Some(Jongseong::NieunJieut),
             0x3135 => Some(Jongseong::NieunJieut),
             0x11AD => Some(Jongseong::NieunHieuh),
             0x3136 => Some(Jongseong::NieunHieuh),
             0x11AE => Some(Jongseong::Digeut),
             0x3137 => Some(Jongseong::Digeut),
             0x11AF => Some(Jongseong::Rieul),
             0x3139 => Some(Jongseong::Rieul),
             0x11B0 => Some(Jongseong::RieulGiyeok),
             0x313A => Some(Jongseong::RieulGiyeok),
             0x11B1 => Some(Jongseong::RieulMieum),
             0x313B => Some(Jongseong::RieulMieum),
             0x11B2 => Some(Jongseong::RieulBieup),
             0x313C => Some(Jongseong::RieulBieup),
             0x11B3 => Some(Jongseong::RieulSiot),
             0x313D => Some(Jongseong::RieulSiot),
             0x11B4 => Some(Jongseong::RieulTieut),
             0x313E => Some(Jongseong::RieulTieut),
             0x11B5 => Some(Jongseong::RieulPieup),
             0x313F => Some(Jongseong::RieulPieup),
             0x11B6 => Some(Jongseong::RieulHieuh),
             0x3140 => Some(Jongseong::RieulHieuh),
             0x11B7 => Some(Jongseong::Mieum),
             0x3141 => Some(Jongseong::Mieum),
             0x11B8 => Some(Jongseong::Bieup),
             0x3142 => Some(Jongseong::Bieup),
             0x11B9 => Some(Jongseong::BieupSiot),
             0x3144 => Some(Jongseong::BieupSiot),
             0x11BA => Some(Jongseong::Siot),
             0x3145 => Some(Jongseong::Siot),
             0x11BB => Some(Jongseong::SsangSiot),
             0x3146 => Some(Jongseong::SsangSiot),
             0x11BC => Some(Jongseong::Ieung),
             0x3147 => Some(Jongseong::Ieung),
             0x11BD => Some(Jongseong::Jieut),
             0x3148 => Some(Jongseong::Jieut),
             0x11BE => Some(Jongseong::Chieut),
             0x314A => Some(Jongseong::Chieut),
             0x11BF => Some(Jongseong::Kieuk),
             0x314B => Some(Jongseong::Kieuk),
             0x11C0 => Some(Jongseong::Tieut),
             0x314C => Some(Jongseong::Tieut),
             0x11C1 => Some(Jongseong::Pieup),
             0x314D => Some(Jongseong::Pieup),
             0x11C2 => Some(Jongseong::Hieuh),
             0x314E => Some(Jongseong::Hieuh),
             _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            Jongseong::Giyeok      => 'ㄱ',
            Jongseong::SsangGiyeok => 'ㄲ',
            Jongseong::GiyeokSiot  => 'ㄳ',
            Jongseong::Nieun       => 'ㄴ',
            Jongseong::NieunJieut  => 'ㄵ',
            Jongseong::NieunHieuh  => 'ㄶ',
            Jongseong::Digeut      => 'ㄷ',
            Jongseong::Rieul       => 'ㄹ',
            Jongseong::RieulGiyeok => 'ㄺ',
            Jongseong::RieulMieum  => 'ㄻ',
            Jongseong::RieulBieup  => 'ㄼ',
            Jongseong::RieulSiot   => 'ㄽ',
            Jongseong::RieulTieut  => 'ㄾ',
            Jongseong::RieulPieup  => 'ㄿ',
            Jongseong::RieulHieuh  => 'ㅀ',
            Jongseong::Mieum       => 'ㅁ',
            Jongseong::Bieup       => 'ㅂ',
            Jongseong::BieupSiot   => 'ㅄ',
            Jongseong::Siot        => 'ㅅ',
            Jongseong::SsangSiot   => 'ㅆ',
            Jongseong::Ieung       => 'ㅇ',
            Jongseong::Jieut       => 'ㅈ',
            Jongseong::Chieut      => 'ㅊ',
            Jongseong::Kieuk       => 'ㅋ',
            Jongseong::Tieut       => 'ㅌ',
            Jongseong::Pieup       => 'ㅍ',
            Jongseong::Hieuh       => 'ㅎ',
        }
    }
}

impl Syllable for Jongseong {
    fn to_u32(&self) -> u32 {
        match *self {
            Jongseong::Giyeok      => 0x11A8,
            Jongseong::SsangGiyeok => 0x11A9,
            Jongseong::GiyeokSiot  => 0x11AA,
            Jongseong::Nieun       => 0x11AB,
            Jongseong::NieunJieut  => 0x11AC,
            Jongseong::NieunHieuh  => 0x11AD,
            Jongseong::Digeut      => 0x11AE,
            Jongseong::Rieul       => 0x11AF,
            Jongseong::RieulGiyeok => 0x11B0,
            Jongseong::RieulMieum  => 0x11B1,
            Jongseong::RieulBieup  => 0x11B2,
            Jongseong::RieulSiot   => 0x11B3,
            Jongseong::RieulTieut  => 0x11B4,
            Jongseong::RieulPieup  => 0x11B5,
            Jongseong::RieulHieuh  => 0x11B6,
            Jongseong::Mieum       => 0x11B7,
            Jongseong::Bieup       => 0x11B8,
            Jongseong::BieupSiot   => 0x11B9,
            Jongseong::Siot        => 0x11BA,
            Jongseong::SsangSiot   => 0x11BB,
            Jongseong::Ieung       => 0x11BC,
            Jongseong::Jieut       => 0x11BD,
            Jongseong::Chieut      => 0x11BE,
            Jongseong::Kieuk       => 0x11BF,
            Jongseong::Tieut       => 0x11C0,
            Jongseong::Pieup       => 0x11C1,
            Jongseong::Hieuh       => 0x11C2,
        }
    }

    fn composable_u32(&self) -> u32 {
        match *self {
            Jongseong::Giyeok      => 1,
            Jongseong::SsangGiyeok => 2,
            Jongseong::GiyeokSiot  => 3,
            Jongseong::Nieun       => 4,
            Jongseong::NieunJieut  => 5,
            Jongseong::NieunHieuh  => 6,
            Jongseong::Digeut      => 7,
            Jongseong::Rieul       => 8,
            Jongseong::RieulGiyeok => 9,
            Jongseong::RieulMieum  => 10,
            Jongseong::RieulBieup  => 11,
            Jongseong::RieulSiot   => 12,
            Jongseong::RieulTieut  => 13,
            Jongseong::RieulPieup  => 14,
            Jongseong::RieulHieuh  => 15,
            Jongseong::Mieum       => 16,
            Jongseong::Bieup       => 17,
            Jongseong::BieupSiot   => 18,
            Jongseong::Siot        => 19,
            Jongseong::SsangSiot   => 20,
            Jongseong::Ieung       => 21,
            Jongseong::Jieut       => 22,
            Jongseong::Chieut      => 23,
            Jongseong::Kieuk       => 24,
            Jongseong::Tieut       => 25,
            Jongseong::Pieup       => 26,
            Jongseong::Hieuh       => 27,
        }
    }
}
