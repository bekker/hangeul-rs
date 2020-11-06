extern crate hangeul;

fn main() {
    // literally: pikachu transliterated
    let subject = "피카츄";

    // Korean marks the topic of the sentence with a post position
    // particle: 이 follows consonants, and 가 follows vowels.
    let post_position = match hangeul::ends_in_consonant(subject).unwrap() {
        true => "이",
        false => "가",
    };

    // -> A wild pikachu has appeared!
    let sentence = format!("야생의 {}{} 나타났다!", subject, post_position);
    println!("{}", sentence); // 야생의 피카츄가 나타났다!

    // get_lead is an alias of get_choseong, to get the first character
    // of a Hangeul syllable.
    let sentence_in_choseong = sentence
        .chars()
        .map(|c| hangeul::get_lead(&c).unwrap_or(c))
        .collect::<String>();

    println!("{}", sentence_in_choseong); // ㅇㅅㅇ ㅍㅋㅊㄱ ㄴㅌㄴㄷ!
}
