# assg1

Implementation and study of Evolutionary Algorithms based on the Travelling salesperson problem.

## Overview
This program is a university assignment for the Artificial intelligence and knowledge engineering course.
The program provides a parser for the TSPLIB file format. The distance matrix constructed by the parser can then be used to find better and better solutions to this TSP instance with the help of an evolutionary algorithm.

The evolutionary algorithm itself can be tweaked to suit the problem's needs. While the basic idea of an evolutionary algorithm remains the same, the user has the ability to adjust its subcomponents, e.g. the selection, crossover, or mutation operators. The program comes with several operators made with TSP in mind, such as OX, PMX, and CX operators for crossover, as well as mutation operators like Swap and Inversion.

For the purposes of the assignment, aside from the evolutionary algorithms, the program also provides random and greedy solution search algorithms.

## Installation

This program is written in [Rust](https://www.rust-lang.org). The recommended way to install Rust is from the official download page. Once you have it set up, try:
```plaintext
git clone https://github.com/karolbelina/siiiw.git
cd siiiw/assg1
cargo run --release -- -vv --input data/ali535.tsp
```

For more information on the usage, try:
```plaintext
cargo run --release -- --help
```
