mod wordle;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
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

    let dictionary = open_dictionary(dictionary_file).unwrap();
    let matches = find_words(dictionary, &wordle);

    for word in matches {
        println!("{}", word);
    }
}

fn open_dictionary(dictionary_file: String) -> Result<Box<dyn BufRead>, Error> {
    if dictionary_file == "-" {
        Ok(Box::new(io::stdin().lock()))
    } else {
        let file: File = File::open(dictionary_file)?;
        Ok(Box::new(BufReader::new(file)))
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
