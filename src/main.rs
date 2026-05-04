mod wordle;
mod dictionary;

use std::env;
use clap::Parser;
use crate::dictionary::Dictionary;
use crate::wordle::Wordle;

const DEFAULT_DICTIONARY_FILE: &str = "/usr/share/dict/american-english";

#[derive(Parser)]
struct CliArgs {
    #[arg(short = 'd', long = "dictionary")]
    dictionary_file: Option<String>,
    correct: String,
    misplaced: Option<String>,
    incorrect: Option<String>
}

fn main() {
    let args: CliArgs = CliArgs::parse();

    let dictionary_file: String = args.dictionary_file
        .or_else(|| env::var("DICTIONARY").ok())
        .unwrap_or(String::from(DEFAULT_DICTIONARY_FILE));

    let wordle: Wordle = Wordle::create(args.correct, args.misplaced, args.incorrect).unwrap();

    println!("{}", wordle);

    let matches: Vec<String> = Dictionary::new(&dictionary_file)
        .select(|word| wordle.matches(word))
        .unwrap();

    for word in matches {
        println!("{}", word);
    }
}
