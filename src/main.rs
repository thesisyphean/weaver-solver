mod words;
use crate::words::WORDS;

use std::error::Error;
use std::io::{self, Write};
use std::collections::{HashMap, VecDeque};

use clap::Parser;
use spinners::{Spinner, Spinners};

/// A program to solve weaver problems
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The word to start at
    #[clap(value_parser)]
    start: String,

    /// The word to end at
    #[clap(value_parser)]
    end: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("{:?}", args);

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

    let mut spinner = Spinner::new(Spinners::Line, "Precomputing words.".into());
    let hashmap = precompute();
    spinner.stop();

    let mut spinner = Spinner::new(Spinners::Line, "Solving weaver.".into());
    //let result = solve(hashmap, starting_word.clone(), ending_word.clone());
    let test = breadth_first_search(hashmap, starting_word, ending_word);
    spinner.stop();
    println!("{:?}", test);

    // print!("{} -> ", starting_word);
    // for word in result {
    //     print!("{} -> ", word);
    // }
    // println!("{}", ending_word);

    Ok(())
}

type WordMap = HashMap<&'static str, Vec<&'static str>>;
    
fn solve(hashmap: WordMap, starting_word: String, ending_word: String) -> Vec<&'static str> {
    let map = breadth_first_search(hashmap, starting_word.clone(), ending_word.clone());
    let mut solution = Vec::new();

    let mut current_word: &str = &ending_word;
    loop {
        let next_word = *map.get(&current_word).unwrap();
        solution.push(next_word);
        current_word = next_word;

        if current_word == "starting_word" {
            break;
        }
    }

    solution
}

fn breadth_first_search<'a>(hashmap: WordMap, starting_word: String, ending_word: String)
    -> HashMap<&'static str, &'static str> {
    let mut previous_word = HashMap::new();
    let mut queue: VecDeque<&str> = VecDeque::new();

    let starting_word: &str = &starting_word;
    for &word in hashmap.get(&starting_word).unwrap() {
        queue.push_back(word);
        previous_word.insert(word, "starting_word");
    }

    while !queue.is_empty() {
        let new_word = queue.pop_front().unwrap();
        if new_word == &ending_word {
            return previous_word;
        }

        for &word in hashmap.get(new_word).unwrap() {
            queue.push_back(word);
            previous_word.insert(word, new_word);
        }
    }

    unreachable!();
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
