# assg1

Implementation of evolutionary algorithms based on the Travelling salesperson problem.

## Overview

This program is a university assignment for the Artificial intelligence and knowledge engineering course. The program provides a parser for the [TSPLIB file format](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/tsp95.pdf). The distance matrix constructed by the parser can then be used to find better and better solutions to this TSP instance with the help of an evolutionary algorithm.

The evolutionary algorithm itself can be tweaked to suit the problem's needs. While the basic idea of an evolutionary algorithm remains the same, the user has the ability to adjust its subcomponents, e.g. the selection, crossover, or mutation operators. The program comes with several operators made with TSP in mind, such as OX and CX operators for crossover, as well as mutation operators like Swap and Inversion.

For the purposes of the assignment, aside from the evolutionary algorithms, the program also provides random and greedy solution search algorithms.

## Installation

- Ensure that [Rust](https://www.rust-lang.org/tools/install) is installed.
- Build and run the program with
  ```bash
  cargo run --release -- -vv --input data/ali535.tsp
  ```
  where the `--input` argument is one of the several TSPLIB files that come with the program.

## Tweaking

The [`main.rs`](https://github.com/karolbelina/siiiw/blob/master/assg1/src/main.rs) file provides a basic setup for the evolutionary algorithm. There, the operators or hyperparameters of the EA can be changed to suit the problem's needs. Additional operators can be found in the [`ops`](https://github.com/karolbelina/siiiw/blob/master/assg1/src/tsp/ops.rs) module. The program provides no tweaking from the command line argument level aside from the input and output file paths.