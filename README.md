# hangeul-rs

[![Build Status](https://travis-ci.org/bekker/hangeul-rs.svg?branch=master)](https://travis-ci.org/bekker/hangeul-rs)

Korean alphabet manipulation library for Rust.

It is lightweight, without any unicode libraries.

```toml
[dependencies]
hangeul = { git = "https://github.com/bekker/hangeul-rs.git" }
```

## Usage

```rust
extern crate hangeul;

fn main() {
    let subjective = "피카츄";
    let sub_josa = match hangeul::ends_with_jongseong(subjective).unwrap() {
        true => "이",
        false => "가"
    };
    let sentence = format!("야생의 {}{} 나타났다!", subjective, sub_josa);
    println!("{}", sentence); // 야생의 피카츄가 나타났다!
    print_choseong(&sentence); // ㅇㅅㅇ ㅍㅋㅊㄱ ㄴㅌㄴㄷ!
}

fn print_choseong(s:&str) {
    for c in s.chars() {
        print!("{}", hangeul::get_choseong(c).unwrap_or(c));
    }
    print!("\n");
}
```

## Why not 'Hangul'?
'Hangul' is from old romanization system - McCune–Reischauer system.

'Hangeul' is official in South Korea, since 2000

## License
Distributed under MIT License
