use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub struct Wordle {
    correct: String,
    misplaced: String,
    incorrect: HashSet<char>
}

impl Wordle {
    pub fn create(correct: Option<String>, misplaced: Option<String>, incorrect: Option<String>) -> Result<Wordle, String> {
        let correct: String = Self::process_correct(correct)?;
        let misplaced: String = Self::process(misplaced);
        let incorrect: HashSet<char> = HashSet::from_iter(Self::process(incorrect).chars());

        if misplaced.len() > correct.len() {
            Err(format!("Too many misplaced letters: {}", misplaced))
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
        let incorrect: String = self.incorrect.iter().collect();

        formatter.write_str(
            format!("Correct letters: {}\nMisplaced Letters: {}\nIncorrect Letters: {}",
                    self.correct, self.misplaced, incorrect)
                .as_str())
    }
}
