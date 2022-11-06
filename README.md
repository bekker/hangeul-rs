# hangeul-rs

[![Build Status](https://travis-ci.org/bekker/hangeul-rs.svg?branch=master)](https://travis-ci.org/bekker/hangeul-rs)
[![docs badge](https://docs.rs/hangeul/badge.svg)](https://docs.rs/hangeul)
[![crates version badge](https://img.shields.io/crates/v/hangeul.svg)](https://crates.io/crates/hangeul)

A lightweight Korean alphabet manipulation library for Rust. No external crates are used.

Only modern, composable Korean unicode blocks are supported (`AC00`–`D7AF`, `1100`–`11FF`, `3130`–`318F`).

```toml
[dependencies]
hangeul = "0.3.0"
```

## Usage

```rust
extern crate hangeul;

fn main() {
    // literally: pikachu transliterated
    let subject = "피카츄";

    // Korean marks the topic of the sentence with a post position
    // particle: 이 follows consonants, and 가 follows vowels.
    let post_position = match hangeul::ends_in_consonant(subject).unwrap() {
        true => "이",
        false => "가"
    };

    // -> A wild pikachu has appeared!
    let sentence = format!("야생의 {}{} 나타났다!", subject, post_position);
    println!("{}", sentence); // 야생의 피카츄가 나타났다!

    // get_lead is an alias of get_choseong, to get the first character
    // of a Hangeul syllable.
    let sentence_in_choseong = sentence.chars()
                                .map(|c| hangeul::get_lead(&c).unwrap_or(c))
                                .collect::<String>();

    println!("{}", sentence_in_choseong); // ㅇㅅㅇ ㅍㅋㅊㄱ ㄴㅌㄴㄷ!
}
```

## Examples
[Examples](./examples).

## Documentation
[Docs.rs](https://docs.rs/hangeul/)

## License
Distributed under MIT License
