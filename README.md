# Weaver Solver

This is a solver for the word ladder game [Weaver](https://wordwormdormdork.com/) by wordwormdormdork.

![How to play Weaver](/how_to_play.png)

## Usage

Clone the project and (assuming you have Rust installed) use cargo to run the program with `<starting word> <ending word>` as the commandline arguments (without the angled brackets).
For example, `cargo run --release -- cram pack`. I recommend using the release version as building the word graph takes a little while with debug.


```
$ cargo run -- -h
weaver-solver
A program to solve Weaver problems (a word ladder game by wordwormdormdork)

USAGE:
    weaver-solver <START> <END>

ARGS:
    <START>    The problem's starting word
    <END>      The problem's ending word

OPTIONS:
    -h, --help    Print help information
```

```
$ cargo run --release -- cram pack
Finished precomputing graph.
Finished solving weaver.
Solution: cram -> pram -> prat -> peat -> peak -> peck -> pack
Optimal length: 6
```

## Solution

The program contains the list of possible words that Weaver allows, from which it builds a graph in which any words that are one letter apart are joined by an edge. It then simply performs breadth-first search from the start word to the end word in order to find and print the optimal path.

### Improvements

The graph generation could be massively sped up by a smarter search through the list of words after sorting them. The $O(n^2)$ solution runs in less than a second though so I haven't written a better method, but I might come back to it in future. An even better way to build the graph could be to originally store the words in a trie instead of a vector.
