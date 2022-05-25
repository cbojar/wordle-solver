use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;
use std::iter::FromIterator;

const DEFAULT_DICTIONARY_FILE: &str = "/usr/share/dict/american-english";

fn main() {
    let dictionary_file = env::var("DICTIONARY").ok()
        .map(|value| String::from(value.trim()))
        .filter(|value| !value.is_empty())
        .unwrap_or(String::from(DEFAULT_DICTIONARY_FILE));
    let correct_letters = env::args().nth(1)
        .map(|value| String::from(value.trim()).to_lowercase())
        .filter(|value| !value.is_empty())
        .expect("No correct letters given");
    let misplaced_letters = env::args().nth(2)
        .map(|value| String::from(value.trim()).to_lowercase())
        .unwrap_or(String::from(""));
    let incorrect_letters = env::args().nth(3)
        .map(|value| String::from(value.trim()).to_lowercase())
        .unwrap_or(String::from(""));

    if misplaced_letters.chars().count() > correct_letters.chars().count() {
        panic!("Too many misplaced letters: {}", misplaced_letters);
    }

    println!("Correct letters: {}", correct_letters);
    println!("Misplaced letters: {}", misplaced_letters);
    println!("Incorrect letters: {}", incorrect_letters);

    let dictionary = open_dictionary(&dictionary_file);
    let matches = find_words(
            dictionary,
            &correct_letters,
            &misplaced_letters,
            &HashSet::from_iter(incorrect_letters.chars()))
        .expect("Failed to read dictionary file");
    
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

fn find_words(
        dictionary: Box<dyn BufRead>,
        correct_letters: &String,
        misplaced_letters: &String,
        incorrect_letters: &HashSet<char>
) -> io::Result<Vec<String>> {
    let mut matches: Vec<String> = Vec::new();

    for line in dictionary.lines() {
        let safe_line = line?.clone();
        if safe_line.chars().count() == correct_letters.chars().count()
                && !has_invalid_chars(&safe_line)
                && matches_correct_letters(correct_letters, &safe_line)
                && matches_misplaced_letters(misplaced_letters, &safe_line)
                && !matches_incorrect_letters(incorrect_letters, &safe_line) {
            matches.push(safe_line);
        }
    }

    return Ok(matches);
}

fn has_invalid_chars(word: &String) -> bool {
    return word.chars().any(|char| char < 'a' || char > 'z');
}

fn matches_correct_letters(correct_letters: &String, word: &String) -> bool {
    return word.chars()
        .zip(correct_letters.chars())
        .filter(|(_char, correct_char)| correct_char != &'_')
        .all(|(char, correct_char)| char == correct_char)
}

fn matches_misplaced_letters(misplaced_letters: &String, word: &String) -> bool {
    return misplaced_letters.chars()
        .all(|misplaced_char| word.chars().any(|char| char == misplaced_char));
}

fn matches_incorrect_letters(incorrect_letters: &HashSet<char>, word: &String) -> bool {
    return word.chars().any(|char| incorrect_letters.contains(&char));
}
