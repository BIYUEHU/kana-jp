use kana_jp;
use std::io;
use utils::lib;

mod utils;
use utils::color;

fn main() {
    // color::red(text)
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{} => {}", input, kana_jp::to_romaji(&input))
}
