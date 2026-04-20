use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};

pub struct Dictionary {
    filename: String,
}

impl Dictionary {
    pub fn new(filename: &String) -> Dictionary {
        Dictionary { filename: filename.to_owned() }
    }

    pub fn select<P>(&self, predicate: P) -> Result<Vec<String>, Error> where P: Fn(&String) -> bool {
        let source = self.open()?;
        Ok(Self::read_words(source).filter(|word| predicate(word)).collect())
    }

    fn open(&self) -> Result<Box<dyn BufRead>, Error> {
        if self.filename == "-" {
            Ok(Box::new(io::stdin().lock()))
        } else {
            let file: File = File::open(&self.filename)?;
            Ok(Box::new(BufReader::new(file)))
        }
    }

    fn read_words(source: Box<dyn BufRead>) -> impl Iterator<Item=String> {
        source
            .lines()
            .map(|line| line.unwrap_or(String::new()))
            .filter(|line| !line.is_empty())
            .filter(|line| !Self::has_invalid_chars(line))
    }

    fn has_invalid_chars(word: &String) -> bool {
        word.chars()
            .flat_map(char::to_lowercase)
            .any(|c| !('a'..='z').contains(&c))
    }
}
