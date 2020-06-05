# assg3

Connect Four game simulator and an implementation of zero-sum game solving algorithms.

Check out the most recent live version at [https://karolbelina.github.io/siiiw/assg3/](https://karolbelina.github.io/siiiw/assg3/).

## Overview

This game is a university assignment for the Artificial intelligence and knowledge engineering course. It is a simulation of the popular [Connect Four](https://en.wikipedia.org/wiki/Connect_Four) game for two players. The main feature, however, is the implementation of the [Minimax](https://en.wikipedia.org/wiki/Minimax) algorithm and it's extension, [Alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha–beta_pruning), for the AI that the human player, or another AI, can play against.

The parameters of the AI player can be tweaked to set its difficulty. The game provides several different strategies with varying levels of difficulty, as well as their weights for additional flexibility.

## Installation

- Ensure that [Rust](https://www.rust-lang.org/tools/install), [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/), and [NodeJS with npm](https://www.npmjs.com/get-npm) are installed.
- To generate the WASM pkg folder run
  ```bash
  wasm-pack build
  ```
  in the root directory.
- In the www folder, install necessary dependencies with
  ```bash
  cd www
  npm install
  ```
  Then, start up the server with
  ```bash
  npm run start
  ``` 
