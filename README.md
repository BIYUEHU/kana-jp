# kana-jp

A Japanese kana converter library for Rust.

## Usage

```rust
use kana_jp::*;

fn main() {
  assert_eq!(
    get_kanji('夏'),
    Some(map::KanjiData(
      vec![String::from("なつ")],
      vec![String::from("カ"), String::from("ガ"), String::from("ゲ")],
      vec![]
    ))
  );

  assert_eq!(to_hiragana("KI", false), "き");
  assert_eq!(to_hiragana("wa ta shi", false), "わたし");

  assert_eq!(to_katakana("sa", false), "サ");
  assert_eq!(to_katakana("wa ta shi", false), "ワタシ");

  assert_eq!(to_romaji("ナ", false), "na");
  assert_eq!(to_romaji("こんにちは", false), "ko n ni chi ha");

  assert!(is_hiragana("あいうえお"));
  assert!(!is_hiragana("アイウエオ"));
  assert!(!is_hiragana("Hello"));

  assert!(is_katakana("アイウエオ"));
  assert!(!is_katakana("あいうえお"));
  assert!(!is_katakana("World"));

  assert!(is_romaji("a i u e o"));
  assert!(!is_romaji("あいうえお"));
  assert!(!is_romaji("アイウエオ"));

  assert!(is_kana("あいうえお"));
  assert!(is_kana("アイウエオ"));
  assert!(!is_kana("Hello"));

  assert!(is_japanese("こんにちは"));
  assert!(!is_japanese("Hello world"));

  assert!(has_hiragana("こんにちは"));
  assert!(!has_hiragana("KI"));

  assert!(has_katakana("コンニチハ"));
  assert!(!has_katakana("さゆき"));

  assert!(has_kana("こんにちは"));
  assert!(has_kana("コンニチハ"));
  assert!(!has_kana("Hello"));
}
```
