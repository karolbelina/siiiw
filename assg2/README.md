# assg2

Implementation of constraint solvers for Sudoku and Jolka puzzles.

## Overview

This program is a university assignment for the Artificial intelligence and knowledge engineering course. It provides a Constraint solver for Sudoku and Jolka puzzles, where each puzzle has its own representation of variables, values and constraints. The program implements the backtracking algorithm, as well as the backtracking algorithm with forward-checking. It also provides several heuristics for value and variable selection.

Each Sudoku puzzle is parsed from a CSV file, which may contain many different Sudoku boards. Each board is mainly associated with and ID and the initial state of the board, where the order is defined from the top-left corner to the bottom-right corner by rows, and dots represents empty cells.
```plaintext
id;difficulty;puzzle;solution
6;1.0;3......9.4...9.216..7.4......9.51..2.8....3.......9.5..4.....2..1.8....3....164..;
```

Jolka is a crossword puzzle, where the objective is to fill every line on the board with words from a given set. Each Jolka puzzle consists of a board and a word set, where the board is a matrix of `_` and `#` characters representing empty and non-empty cells, and the words set is a list of case-sensitive words separated by line breaks.

## Installation

- Ensure that [Rust](https://www.rust-lang.org/tools/install) is installed.
- To build the program run
  ```bash
  cargo build --release
  ```
- Run the program for a Sudoku puzzle with
  ```bash
  cargo run --release -- -vv -a fc -l lcv -r mcv sudoku data/sudoku.csv 14
  ```
  where `data/sudoku.csv` is the file with 46 various Sudoku puzzles that come with the program, and 14 is the index of the Sudoku within that file.
- Run the program for a Jolka puzzle with
  ```bash
  cargo run --release -- -vv -a fc -l lcv -r mcv jolka data/jolka/puzzle4 data/jolka/words4
  ```
  where `data/jolka/puzzle4` and `data/jolka/words4` are the board and words files of the same Jolka puzzle, respectively.
