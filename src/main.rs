mod wordle;
mod dictionary;
mod cliargs;

use clap::Parser;
use crate::cliargs::CliArgs;
use crate::dictionary::Dictionary;
use crate::wordle::Wordle;

fn main() {
    let args: CliArgs = CliArgs::parse();

    let dictionary: Dictionary = Dictionary::new(args.dictionary_file());
    let wordle: Wordle = Wordle::create(args.correct(), args.misplaced(), args.incorrect()).unwrap();

    println!("{}", wordle);

    let matches: Vec<String> = dictionary.select(|word| wordle.matches(word)).unwrap();

    for word in matches {
        println!("{}", word);
    }
}
