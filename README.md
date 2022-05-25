# wordle-solver

A simple worldle-solver written in Rust.

## How to build

```sh
cargo build --release
```

This will create the binary named `wordle-solver` in a directory called `target/release`.

## How to run

```sh
cd target/release
./wordle-solver <correct letters> [misplaced letters [incorrect letters]]
```

Correct letters is required. The expected value is composed of letters in the correct positions and underscores for
unknown letters. The letters and underscores must match the available characters in the word. For example, a five-letter
word with an "a" in the third position and an "n" in the last position would be entered as `__a_n`.

Misplaced letters is optional, and is simply all the letters that are in the word, but their position is unknown. For
example, if the letters "a", "b", and "r" are in the word, but their position is unknown, the application would be
called as:

```sh
./wordle-solver _____ abr
```

Incorrect letters is also optional, and is simply all the letters that are know to not be in the word. For example, if
the letters "a", "b", and "r" are in the word but their position is unknown, but the letters "t", "s", and "p" are not
in the word, the application would be called as:

```sh
./wordle-solver _____ abr tsp
```

If there are no misplaced letters, the empty string must be passed as the second parameter to not confuse incorrect and
misplaced letters:

```sh
./wordle-sover _____ '' tsp
```

## Dictionary

The default dictionary is located at `/usr/share/dict/american-english`. To use a different dictionary, set the
`DICTIONARY` environment variable to a different file path. If the given file path is empty or only whitespace, the
default dictionary will be used. If the given file path is `-`, the dictionary will be read from standard input.

This application will likely not behave as expected for characters outside of standard English a-z.
