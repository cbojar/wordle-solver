mod wordle;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;
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

    println!("Correct letters: {}", wordle.correct());
    println!("Misplaced letters: {}", wordle.misplaced());
    println!("Incorrect letters: {}", wordle.incorrect());

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
    let mut matches: Vec<String> = Vec::new();
    let incorrect_set = wordle.incorrect_set();

    for line in dictionary.lines().map(|l| l.unwrap_or(String::from(""))) {
        if line.len() == wordle.len()
                && !has_invalid_chars(&line)
                && matches_correct_letters(wordle.correct(), &line)
                && matches_misplaced_letters(wordle.misplaced(), &line)
                && !matches_incorrect_letters(&incorrect_set, &line) {
            matches.push(line);
        }
    }

    matches
}

fn has_invalid_chars(word: &String) -> bool {
    return word.chars().any(|char| char < 'a' || char > 'z');
}

fn matches_correct_letters(correct_letters: &str, word: &String) -> bool {
    return word.chars()
        .zip(correct_letters.chars())
        .filter(|(_char, correct_char)| correct_char != &'_')
        .all(|(char, correct_char)| char == correct_char)
}

fn matches_misplaced_letters(misplaced_letters: &str, word: &String) -> bool {
    return misplaced_letters.chars()
        .all(|misplaced_char| word.chars().any(|char| char == misplaced_char));
}

fn matches_incorrect_letters(incorrect_letters: &HashSet<char>, word: &String) -> bool {
    return word.chars().any(|char| incorrect_letters.contains(&char));
}
