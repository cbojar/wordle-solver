mod wordle;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::wordle::Wordle;

const DEFAULT_DICTIONARY_FILE: &str = "/usr/share/dict/american-english";

fn main() {
    let dictionary_file = env::var("DICTIONARY").ok()
        .map(|value| String::from(value.trim()))
        .filter(|value| !value.is_empty())
        .unwrap_or(String::from(DEFAULT_DICTIONARY_FILE));

    let wordle: Wordle = Wordle::create(
        env::args().nth(1), env::args().nth(2), env::args().nth(3))
        .unwrap();

    println!("{}", wordle);

    let dictionary = open_dictionary(&dictionary_file);
    let matches = find_words(dictionary, &wordle);

    for word in matches {
        println!("{}", word);
    }
}

fn open_dictionary(dictionary_file: &String) -> Box<dyn BufRead> {
    if dictionary_file == "-" {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        return Box::new(reader);
    } else {
        let file = File::open(dictionary_file)
            .expect("Failed to open dictionary");
        let reader = BufReader::new(file);
        return Box::new(reader);
    }
}

fn find_words(dictionary: Box<dyn BufRead>, wordle: &Wordle) -> Vec<String> {
    dictionary.lines()
        .map(|line| line.unwrap_or(String::new()))
        .filter(|line| !has_invalid_chars(line))
        .filter(|line| wordle.matches(line))
        .collect()
}

fn has_invalid_chars(word: &String) -> bool {
    word.chars().any(|char| char < 'a' || char > 'z')
}
