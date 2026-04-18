use std::collections::HashSet;

pub struct Wordle {
    correct: String,
    misplaced: String,
    incorrect: String
}

impl Wordle {
    pub fn create(correct: Option<String>, misplaced: Option<String>, incorrect: Option<String>) -> Result<Wordle, String> {
        let correct: String = Self::process_correct(correct)?;
        let misplaced: String = Self::process(misplaced);
        let incorrect: String = Self::process(incorrect);

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
            .unwrap_or(String::from(""))
    }

    fn normalize(value: String, ok_chars: &str) -> String {
        let mut value: String = String::from(value.trim()).to_lowercase();
        value.retain(|c| ('a'..='z').contains(&c) || ok_chars.contains(c));
        value
    }

    pub fn len(&self) -> usize {
        self.correct.len()
    }

    pub fn correct(&self) -> &str {
        self.correct.as_str()
    }

    pub fn misplaced(&self) -> &str {
        self.misplaced.as_str()
    }

    pub fn incorrect(&self) -> &str {
        self.incorrect.as_str()
    }

    pub fn incorrect_set(&self) -> HashSet<char> {
        HashSet::from_iter(self.incorrect.chars())
    }
}
