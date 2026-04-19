use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

pub struct Wordle {
    correct: String,
    misplaced: String,
    incorrect: BTreeSet<char>
}

impl Wordle {
    pub fn create(correct: Option<String>, misplaced: Option<String>, incorrect: Option<String>) -> Result<Wordle, String> {
        let correct: String = Self::process_correct(correct)?;
        let misplaced: String = Self::process(misplaced);
        let incorrect: BTreeSet<char> = BTreeSet::from_iter(Self::process(incorrect).chars());

        if misplaced.len() > correct.len() {
            Err(format!("Too many misplaced letters: {}", misplaced))
        } else if correct.chars().any(|c| incorrect.contains(&c)) {
            Err(String::from("Correct letter also marked incorrect"))
        } else if misplaced.chars().any(|c| incorrect.contains(&c)) {
            Err(String::from("Misplaced letter also marked incorrect"))
        } else {
            Ok(Wordle { correct, misplaced, incorrect })
        }
    }

    fn process_correct(value: Option<String>) -> Result<String, String> {
        value
            .map(|v| Self::normalize(v,"_"))
            .ok_or_else(|| String::from("No correct letters given"))
    }

    fn process(value: Option<String>) -> String {
        value
            .map(|v| Self::normalize(v, ""))
            .filter(|v| !v.is_empty())
            .unwrap_or(String::new())
    }

    fn normalize(value: String, ok_chars: &str) -> String {
        value.chars()
            .flat_map(char::to_lowercase)
            .filter(|c| ('a'..='z').contains(c) || ok_chars.contains(*c))
            .collect()
    }

    pub fn matches(&self, word: &String) -> bool {
        self.correct.len() == word.len() &&
            self.matches_correct(word) &&
            self.matches_misplaced(word) &&
            !self.matches_incorrect(word)
    }

    fn matches_correct(&self, word: &String) -> bool {
        word.chars()
            .zip(self.correct.chars())
            .filter(|(_, correct)| *correct != '_')
            .all(|(char, correct)| char == correct)
    }

    fn matches_misplaced(&self, word: &String) -> bool {
        self.misplaced.chars()
            .all(|misplaced| word.chars().any(|char| char == misplaced))
    }

    fn matches_incorrect(&self, word: &String) -> bool {
        word.chars().any(|char| self.incorrect.contains(&char))
    }
}

impl Display for Wordle {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter,
               "Correct letters: {}\nMisplaced Letters: {}\nIncorrect Letters: {}",
               self.correct, self.misplaced, self.incorrect.iter().collect::<String>())
    }
}
