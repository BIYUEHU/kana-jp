# kana-jp

A Japanese kana converter library for Rust.

## Usage

```rust
use kana_jp;
fn main() {
    let hiragana = "かな";
    let katakana = kana_jp::to_katakana(hiragana);
    println!("{} -> {}", hiragana, katakana); // Output: かな -> カナ
}
```
