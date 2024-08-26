use crate::map::{
    get_hiragana_to_katakana_map, get_hiragana_to_romaji_map, get_katakana_to_hiragana_map,
    get_katakana_to_romaji_map, get_romaji_to_hiragana_map, get_romaji_to_katakana_map,
};
use std::string::String;

pub use map;

pub fn to_hiragana(text: &str) -> String {
    let mut result = String::new();

    for char in text.chars() {
        match get_romaji_to_hiragana_map().get(String::from(char).as_str()) {
            Some(value) => result.push_str(value),
            None => match get_katakana_to_hiragana_map().get(String::from(char).as_str()) {
                Some(value) => result.push_str(value),
                None => result.push(char),
            },
        }
    }

    result
}

pub fn to_katakana<'a>(text: &str) -> String {
    let mut result = String::new();

    for char in text.chars() {
        match get_romaji_to_katakana_map().get(String::from(char).as_str()) {
            Some(value) => result.push_str(value),
            None => match get_hiragana_to_katakana_map().get(&String::from(char).as_str()) {
                Some(value) => result.push_str(value),
                None => result.push(char),
            },
        }
    }

    result
}

pub fn to_romaji(text: &str) -> String {
    let mut result = String::new();

    for char in text.chars() {
        match get_hiragana_to_romaji_map().get(String::from(char).as_str()) {
            Some(value) => result.push_str(value),
            None => match get_katakana_to_romaji_map().get(&String::from(char).as_str()) {
                Some(value) => result.push_str(value),
                None => result.push(char),
            },
        }
    }

    result
}

pub fn is_hiragana(text: &str) -> bool {
    for char in text.chars() {
        if !get_hiragana_to_romaji_map().contains_key(String::from(char).as_str()) {
            return false;
        }
    }
    true
}

pub fn is_katakana(text: &str) -> bool {
    for char in text.chars() {
        if !get_katakana_to_romaji_map().contains_key(&String::from(char).as_str()) {
            return false;
        }
    }
    true
}

pub fn is_romaji(text: &str) -> bool {
    for char in text.chars() {
        if !get_romaji_to_hiragana_map().contains_key(&String::from(char).as_str()) {
            return false;
        }
    }
    true
}

pub fn is_kana(text: &str) -> bool {
    for char in text.chars() {
        if !get_hiragana_to_romaji_map().contains_key(&String::from(char).as_str())
            && !get_katakana_to_romaji_map().contains_key(&String::from(char).as_str())
        {
            return false;
        }
    }
    true
}

pub fn is_japanese(text: &str) -> bool {
    let mut has_kana = false;

    for char in text.chars() {
        if !get_hiragana_to_romaji_map().contains_key(&String::from(char).as_str())
            && !get_katakana_to_hiragana_map().contains_key(&String::from(char).as_str())
            && !get_romaji_to_hiragana_map().contains_key(&String::from(char).as_str())
        {
            return false;
        }
        if get_hiragana_to_katakana_map().contains_key(&String::from(char).as_str())
            || get_katakana_to_hiragana_map().contains_key(&String::from(char).as_str())
        {
            has_kana = true;
        }
    }

    has_kana
}

pub fn has_hiragana(text: &str) -> bool {
    for char in text.chars() {
        if get_hiragana_to_romaji_map().contains_key(&String::from(char).as_str()) {
            return true;
        }
    }
    false
}

pub fn has_katakana(text: &str) -> bool {
    for char in text.chars() {
        if get_katakana_to_romaji_map().contains_key(&String::from(char).as_str()) {
            return true;
        }
    }
    false
}

pub fn has_kana(text: &str) -> bool {
    for char in text.chars() {
        if get_hiragana_to_katakana_map().contains_key(&String::from(char).as_str())
            || get_katakana_to_hiragana_map().contains_key(&String::from(char).as_str())
        {
            return true;
        }
    }
    false
}
