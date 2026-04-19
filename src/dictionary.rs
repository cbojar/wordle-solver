use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, StdinLock};

pub struct Dictionary {
    words: Vec<String>
}

impl Dictionary {
    pub fn open(filename: &String) -> Result<Dictionary, Error> {
        if filename == "-" {
            Ok(Dictionary::from_stdin())
        } else {
            Dictionary::from_file(filename)
        }
    }

    pub fn from_stdin() -> Dictionary {
        let source: Box<StdinLock> = Box::new(io::stdin().lock());
        let words: Vec<String> = Self::read_words(source);
        Dictionary { words }
    }

    pub fn from_file(filename: &String) -> Result<Dictionary, Error> {
        let file: File = File::open(filename)?;
        let source: Box<BufReader<File>> = Box::new(BufReader::new(file));
        let words: Vec<String> = Self::read_words(source);
        Ok(Dictionary { words })
    }

    fn read_words(source: Box<dyn BufRead>) -> Vec<String> {
        source
            .lines()
            .map(|line| line.unwrap_or(String::new()))
            .filter(|line| !line.is_empty())
            .filter(|line| !Self::has_invalid_chars(line))
            .collect()
    }

    fn has_invalid_chars(word: &String) -> bool {
        word.chars()
            .flat_map(char::to_lowercase)
            .any(|char| char < 'a' || char > 'z')
    }

    pub fn words(&self) -> impl Iterator<Item=&String> {
        self.words.iter()
    }
}
