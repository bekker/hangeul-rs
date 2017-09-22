# hangeul-rs

[![Build Status](https://travis-ci.org/bekker/hangeul-rs.svg?branch=master)](https://travis-ci.org/bekker/hangeul-rs)

[![](http://meritbadge.herokuapp.com/hangeul)](https://crates.io/crates/hangeul)

Korean alphabet manipulation library for Rust.

It is lightweight, without any unicode libraries.

Supports only modern korean alphabets.

```toml
[dependencies]
hangeul = "0.1.3"
```

## Usage

```rust
extern crate hangeul;

fn main() {
    let subjective = "피카츄";
    let suffix = match hangeul::ends_with_jongseong(subjective).unwrap() {
        true => "이",
        false => "가"
    };

    let sentence = format!("야생의 {}{} 나타났다!", subjective, suffix);
    println!("{}", sentence); // 야생의 피카츄가 나타났다!

    let sentence_in_choseong = sentence.chars()
                                .map(|c| hangeul::get_choseong(c).unwrap_or(c))
                                .collect::<String>();
    println!("{}", sentence_in_choseong); // ㅇㅅㅇ ㅍㅋㅊㄱ ㄴㅌㄴㄷ!
}
```

## Documentation
[Docs.rs](https://docs.rs/hangeul/)

## License
Distributed under MIT License
