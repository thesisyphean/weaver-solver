mod words;
use crate::words::WORDS;

use std::error::Error;
use std::collections::{HashMap, VecDeque};

use clap::Parser;
use spinners::{Spinner, Spinners};

/// A program to solve weaver problems
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The word to start at
    #[clap(value_parser = valid_word)]
    start: String,

    /// The word to end at
    #[clap(value_parser = valid_word)]
    end: String,
}

fn valid_word(word: &str) -> Result<String, String> {
    if word.chars().count() != 4 {
        Err("Words must be four letters long".into())
    } else {
        if !WORDS.contains(&word) {
            Err("Words must be valid english words".into())
        } else {
            Ok(word.to_string())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    args.start.make_ascii_lowercase();
    args.end.make_ascii_lowercase();
    let start = WORDS.iter().position(|&w| w == &args.start).unwrap();
    let end = WORDS.iter().position(|&w| w == &args.end).unwrap();

    println!("Welcome to Weaver Solver!");

    let mut spinner = Spinner::new(Spinners::Line, "Precomputing words.".into());
    let hashmap = generate_graph();
    spinner.stop();

    let mut spinner = Spinner::new(Spinners::Line, "Solving weaver.".into());
    let result = breadth_first_search(hashmap, start, end);
    let solution = solve(result, start, end);
    spinner.stop();

    print!("{} -> ", args.start);
    for word in solution {
        print!("{} -> ", word);
    }
    println!("{}", args.end);

    Ok(())
}

fn solve(pred: Vec<usize>, start: usize, end: usize) -> Vec<&'static str> {
    let mut solution = Vec::new();

    let mut new_word = end;
    loop {
        new_word = pred[new_word];

        if new_word == start {
            break;
        }

        solution.push(WORDS[new_word]);
    }

    solution.reverse();
    solution
}

fn breadth_first_search(graph: HashMap<usize, Vec<usize>>, start: usize, end: usize) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut visited: Vec<_> = (0..WORDS.len()).map(|_| false).collect();
    let mut pred: Vec<_> = (0..WORDS.len()).map(|_| 0).collect();

    visited[start] = true;
    queue.push_back(start);

    while !queue.is_empty() {
        let new_word = queue.pop_front().unwrap();

        for &adj_word in &graph[&new_word] {
            if !visited[adj_word] {
                visited[adj_word] = true;
                pred[adj_word] = new_word;

                queue.push_back(adj_word);

                if adj_word == end {
                    return pred;
                }
            }
        }
    }

    unreachable!();
}

fn generate_graph() -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();

    for i in 0..WORDS.len() {
        graph.insert(i, Vec::new());
    }

    for i in 0..WORDS.len() {
        for j in i + 1..WORDS.len() {
            if matches(i, j) {
                graph.get_mut(&i).unwrap().push(j);
                graph.get_mut(&j).unwrap().push(i);
            }
        }
    }

    graph
}

fn matches(i: usize, j: usize) -> bool {
    let achars: Vec<_> = WORDS[i].chars().collect();
    let bchars: Vec<_> = WORDS[j].chars().collect();

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
