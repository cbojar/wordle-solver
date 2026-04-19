mod wordle;
mod dictionary;

use std::env;
use crate::dictionary::Dictionary;
use crate::wordle::Wordle;

const DEFAULT_DICTIONARY_FILE: &str = "/usr/share/dict/american-english";

fn main() {
    let dictionary_file = env::var("DICTIONARY").ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(String::from(DEFAULT_DICTIONARY_FILE));

    let wordle: Wordle = Wordle::create(
        env::args().nth(1),
        env::args().nth(2),
        env::args().nth(3)
    ).unwrap();

    println!("{}", wordle);

    let dictionary: Dictionary = Dictionary::open(&dictionary_file).unwrap();

    for word in dictionary.words().filter(|word| wordle.matches(word)) {
        println!("{}", word);
    }
}
