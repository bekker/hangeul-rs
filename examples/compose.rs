extern crate hangeul;

fn main() {
    let cho = 'ㄱ';
    let jung = 'ㅏ';
    let jong = Some(&'ㅄ');

    let composed = hangeul::compose_char(&cho, &jung, jong).unwrap();
    assert_eq!(composed, '값');

    let (cho2, jung2, jong2) = hangeul::decompose_char(&composed).unwrap();
    assert_eq!(cho, cho2);
    assert_eq!(jung, jung2);
    assert_eq!(jong.unwrap(), &jong2.unwrap());
}
