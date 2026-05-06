use std::env;
use clap::Parser;

const DEFAULT_DICTIONARY_FILE: &str = "/usr/share/dict/american-english";

#[derive(Parser)]
pub struct CliArgs {
    #[arg(short = 'd', long = "dictionary")]
    dictionary_file: Option<String>,
    correct: String,
    misplaced: Option<String>,
    incorrect: Option<String>
}

impl CliArgs {
    pub fn dictionary_file(&self) -> String {
        self.dictionary_file.as_ref()
            .map(|f| f.to_owned())
            .or_else(|| env::var("DICTIONARY").ok())
            .unwrap_or(String::from(DEFAULT_DICTIONARY_FILE))
    }

    pub fn correct(&self) -> String {
        self.correct.to_owned()
    }

    pub fn misplaced(&self) -> String {
        self.misplaced.as_ref()
            .map(|m| m.to_owned())
            .unwrap_or_else(String::new)
    }

    pub fn incorrect(&self) -> String {
        self.incorrect.as_ref()
            .map(|i| i.to_owned())
            .unwrap_or_else(String::new)
    }
}
