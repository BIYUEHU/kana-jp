pub mod map;
use map::{
    get_hiragana_to_katakana_map, get_hiragana_to_romaji_map, get_kanji_map,
    get_katakana_to_hiragana_map, get_katakana_to_romaji_map, get_romaji_to_hiragana_map,
    get_romaji_to_katakana_map, KanjiData,
};
use std::string::String;

fn handle_kanji_factory<'a>(kanji: bool) -> impl Fn(char) -> String {
    if kanji {
        |x: char| {
            if let Some(KanjiData(kun, on, nanori)) = get_kanji(x) {
                kun.first()
                    .or_else(|| on.first())
                    .or_else(|| nanori.first())
                    .cloned()
                    .unwrap_or_else(|| x.to_string())
                    .replace(".", "")
            } else {
                x.to_string()
            }
        }
    } else {
        |x: char| x.to_string()
    }
}

fn handle_char_factory(mapping: map::KanaMap) -> impl Fn(String) -> String {
    let handle = move |str: String| {
        mapping
            .get(str.clone().as_str())
            .cloned()
            .unwrap_or(str.to_string().as_str())
            .to_string()
    };
    move |str: String| match str.len() {
        0 => str,
        1 => handle(str),
        _ => str
            .chars()
            .map(|c| handle(c.to_string()))
            .collect::<String>(),
    }
}

pub fn get_kanji(char: char) -> Option<KanjiData> {
    let code_point = char as u32;
    let hex_string = format!("{:x}", code_point);
    get_kanji_map().get(hex_string.as_str()).cloned()
}

pub fn to_hiragana(text: &str, kanji: bool) -> String {
    let romaji_to_hiragana_map = get_romaji_to_hiragana_map();
    let handle_kanji = handle_kanji_factory(kanji);
    let handle_char = handle_char_factory(get_katakana_to_hiragana_map());

    text.to_lowercase()
        .split(' ')
        .map(|x| {
            if let Some(value) = romaji_to_hiragana_map.get(String::from(x).as_str()) {
                return value.to_string();
            }
            x.chars()
                .map(&handle_kanji)
                .map(&handle_char)
                .collect::<String>()
                .to_string()
        })
        .collect()
}

pub fn to_katakana<'a>(text: &str, kanji: bool) -> String {
    let romaji_to_katakana_map = get_romaji_to_katakana_map();
    let handle_kanji = handle_kanji_factory(kanji);
    let handle_char = handle_char_factory(get_hiragana_to_katakana_map());

    text.to_lowercase()
        .split(' ')
        .map(|x| {
            if let Some(value) = romaji_to_katakana_map.get(String::from(x).as_str()) {
                return value.to_string();
            }
            x.chars()
                .map(&handle_kanji)
                .map(&handle_char)
                .collect::<String>()
                .to_string()
        })
        .collect()
}

pub fn to_romaji(text: &str, kanji: bool) -> String {
    let hiragana_to_romaji_map = get_hiragana_to_romaji_map();
    let katakana_to_romaji_map = get_katakana_to_romaji_map();

    String::from(
        text.to_lowercase()
            .chars()
            .map(handle_kanji_factory(kanji))
            .collect::<String>()
            .chars()
            .map(|x| {
                if let Some(value) = hiragana_to_romaji_map.get(String::from(x).as_str()) {
                    format!("{} ", value)
                } else if let Some(value) = katakana_to_romaji_map.get(String::from(x).as_str()) {
                    format!("{} ", value)
                } else {
                    x.to_string()
                }
            })
            .collect::<String>()
            .trim(),
    )
}

pub fn is_hiragana(text: &str) -> bool {
    let get_hiragana_to_romaji_map = get_hiragana_to_romaji_map();

    for char in text.to_lowercase().chars() {
        if !get_hiragana_to_romaji_map.contains_key(String::from(char).as_str()) {
            return false;
        }
    }

    true
}

pub fn is_katakana(text: &str) -> bool {
    let get_katakana_to_romaji_map = get_katakana_to_romaji_map();

    for char in text.to_lowercase().chars() {
        if !get_katakana_to_romaji_map.contains_key(&String::from(char).as_str()) {
            return false;
        }
    }

    true
}

pub fn is_romaji(text: &str) -> bool {
    let romaji_to_hiragana_map = get_romaji_to_hiragana_map();

    for char in text.to_lowercase().split(' ') {
        if !romaji_to_hiragana_map.contains_key(&String::from(char).as_str()) {
            return false;
        }
    }

    true
}

pub fn is_kana(text: &str) -> bool {
    let get_hiragana_to_romaji_map = get_hiragana_to_romaji_map();
    let get_katakana_to_romaji_map = get_katakana_to_romaji_map();

    for char in text.to_lowercase().chars() {
        if !get_hiragana_to_romaji_map.contains_key(&String::from(char).as_str())
            && !get_katakana_to_romaji_map.contains_key(&String::from(char).as_str())
        {
            return false;
        }
    }

    true
}

pub fn is_japanese(text: &str) -> bool {
    let mut has_kana = false;
    let hiragana_to_romaji_map = get_hiragana_to_romaji_map();
    let katakana_to_hiragana_map = get_katakana_to_hiragana_map();
    let romaji_to_hiragana_map = get_romaji_to_hiragana_map();
    let hiragana_to_katakana_map = get_hiragana_to_katakana_map();

    for char in text.to_lowercase().chars() {
        if !hiragana_to_romaji_map.contains_key(&String::from(char).as_str())
            && !katakana_to_hiragana_map.contains_key(&String::from(char).as_str())
            && !romaji_to_hiragana_map.contains_key(&String::from(char).as_str())
        {
            return false;
        }
        if hiragana_to_katakana_map.contains_key(&String::from(char).as_str())
            || katakana_to_hiragana_map.contains_key(&String::from(char).as_str())
        {
            has_kana = true;
        }
    }

    has_kana
}

pub fn has_hiragana(text: &str) -> bool {
    let hiragana_to_romaji_map = get_hiragana_to_romaji_map();

    for char in text.to_lowercase().chars() {
        if hiragana_to_romaji_map.contains_key(&String::from(char).as_str()) {
            return true;
        }
    }

    false
}

pub fn has_katakana(text: &str) -> bool {
    let katakana_to_romaji_map = get_katakana_to_romaji_map();

    for char in text.to_lowercase().chars() {
        if katakana_to_romaji_map.contains_key(&String::from(char).as_str()) {
            return true;
        }
    }

    false
}

pub fn has_kana(text: &str) -> bool {
    let hiragana_to_katakana_map = get_hiragana_to_katakana_map();
    let katakana_to_hiragana_map = get_katakana_to_hiragana_map();

    for char in text.to_lowercase().chars() {
        if hiragana_to_katakana_map.contains_key(&String::from(char).as_str())
            || katakana_to_hiragana_map.contains_key(&String::from(char).as_str())
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_et_kanji() {
        assert_eq!(
            get_kanji('夏'),
            Some(KanjiData(
                vec![String::from("なつ")],
                vec![String::from("カ"), String::from("ガ"), String::from("ゲ")],
                vec![]
            ))
        )
    }

    #[test]
    fn test_to_hiragana() {
        // assert_eq!(to_hiragana("お元気です"), "おおもときです");
        assert_eq!(to_hiragana("KI", false), "き");
        assert_eq!(to_hiragana("wa ta shi", false), "わたし");
    }

    #[test]
    fn test_to_katakana() {
        // assert_eq!(to_katakana("好きです"), "スイキです");
        assert_eq!(to_katakana("sa", false), "サ");
        assert_eq!(to_katakana("wa ta shi", false), "ワタシ");
    }

    #[test]
    fn test_to_romaji() {
        // assert_eq!(to_romaji("夏はいい天気です"), "natsu ha i i tenki desu");
        assert_eq!(to_romaji("ナ", false), "na");
        assert_eq!(to_romaji("こんにちは", false), "ko n ni chi ha");
    }

    #[test]
    fn test_is_hiragana() {
        assert!(is_hiragana("あいうえお"));
        assert!(!is_hiragana("アイウエオ"));
        assert!(!is_hiragana("Hello"));
    }

    #[test]
    fn test_is_katakana() {
        assert!(is_katakana("アイウエオ"));
        assert!(!is_katakana("あいうえお"));
        assert!(!is_katakana("World"));
    }

    #[test]
    fn test_is_romaji() {
        assert!(is_romaji("a i u e o"));
        assert!(!is_romaji("あいうえお"));
        assert!(!is_romaji("アイウエオ"));
    }

    #[test]
    fn test_is_kana() {
        assert!(is_kana("あいうえお"));
        assert!(is_kana("アイウエオ"));
        assert!(!is_kana("Hello"));
    }

    #[test]
    fn test_is_japanese() {
        assert!(is_japanese("こんにちは"));
        assert!(!is_japanese("Hello world"));
    }

    #[test]
    fn test_has_hiragana() {
        assert!(has_hiragana("こんにちは"));
        assert!(!has_hiragana("KI"));
    }

    #[test]
    fn test_has_katakana() {
        assert!(has_katakana("コンニチハ"));
        assert!(!has_katakana("さゆき"));
    }

    #[test]
    fn test_has_kana() {
        assert!(has_kana("こんにちは"));
        assert!(has_kana("コンニチハ"));
        assert!(!has_kana("Hello"));
    }
}
