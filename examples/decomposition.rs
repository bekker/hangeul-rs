extern crate hangeul;

fn main() {
    let subjective = "피카츄";
    let sub_josa = match hangeul::ends_with_jongseong(subjective).unwrap() {
        true => "이",
        false => "가"
    };
    let sentence = format!("야생의 {}{} 나타났다!", subjective, sub_josa);
    println!("{}", sentence); // 야생의 피카츄가 나타났다!
    print_choseng(&sentence); // ㅇㅅㅇ ㅍㅋㅊㄱ ㄴㅌㄴㄷ!
}

fn print_choseng(s:&str) {
    for c in s.chars() {
        print!("{}", hangeul::get_choseong(c).unwrap_or(c));
    }
    print!("\n");
}