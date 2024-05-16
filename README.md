# Weaver Solver

This is a solver of the word ladder game [Weaver](https://wordwormdormdork.com/).
It takes the list of possible words and creates a graph in which connections between words indicate that they are one letter apart.
It then simply performs depth-first search to find the shortest path between the first and last word.

## Execution

Clone the project and use cargo to run the program with `<starting word> <ending word>` as the commandline arguments (without the <>).
For example, `cargo run cram pack`.
