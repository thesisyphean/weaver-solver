mod words;
use crate::words::WORDS;

use std::error::Error;
use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Weaver Solver!");

    let mut input = String::new();
    print!("Starting word: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let starting_word = input.trim().to_string();

    input.clear();
    print!("Ending word: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let ending_word = input.trim().to_string();

    println!("Precomputing word matches.");
    let hashmap = precompute();

    println!("Solving weaver.");
    let result = solve(hashmap, &starting_word, &ending_word);
    println!("Solution: {:?}", result);

    Ok(())
}

type WordMap = HashMap<&'static str, Vec<&'static str>>;

// Args
// Colour
// Spinner while loading

fn solve(hashmap: WordMap, starting_word: &str, ending_word: &str) -> Vec<&'static str> {
    let mut solution = Vec::new();
    let mut current_word = starting_word;

    loop {
        let possible_words = hashmap.get(current_word).unwrap();
        
    }

    solution
}

fn precompute() -> WordMap {
    let mut hashmap = HashMap::new();

    for word in WORDS {
        hashmap.insert(word, Vec::new());
    }

    for i in 0..WORDS.len() {
        let from_word = WORDS[i];
        for j in i + 1..WORDS.len() {
            let to_word = WORDS[j];
            if matches(from_word, to_word) {
                hashmap.get_mut(from_word).unwrap().push(to_word);
                hashmap.get_mut(to_word).unwrap().push(from_word);
            }
        }
    }

    hashmap
}

fn matches(a: &str, b: &str) -> bool {
    let achars: Vec<_> = a.chars().collect();
    let bchars: Vec<_> = b.chars().collect();

    let mut one_off = false;
    for i in 0..4 {
        let achar = achars[i];
        let bchar = bchars[i];
        if achar != bchar {
            if one_off {
                return false;
            } else {
                one_off = true;
            }
        }
    }

    true
}
