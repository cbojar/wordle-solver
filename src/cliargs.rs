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
            .unwrap_or_else(|| String::from(DEFAULT_DICTIONARY_FILE))
    }

    pub fn correct(&self) -> &str {
        self.correct.as_str()
    }

    pub fn misplaced(&self) -> &str {
        self.misplaced.as_ref()
            .map(|m| &m[..])
            .unwrap_or("")
    }

    pub fn incorrect(&self) -> &str {
        self.incorrect.as_ref()
            .map(|i| &i[..])
            .unwrap_or("")
    }
}
