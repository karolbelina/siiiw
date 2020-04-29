use super::{Game, Node};
use std::i32;
use std::cmp::{min, max};

pub fn minimax<G: Game, F>(node: &G::State, depth: usize, maximizing: bool, eval: F)
    -> Option<<G::State as Node<G>>::Decision>
where
    F: Fn(&G::State) -> i32 + Copy
{
    fn aux<G: Game, F>(node: &G::State, depth: usize, maximizing: bool, eval: F) -> i32
    where
        F: Fn(&G::State) -> i32 + Copy
    {
        if depth == 0 || node.is_terminal() {
            return eval(node);
        }
        if maximizing {
            let mut best = i32::MIN;
            for decision in node.decisions(maximizing) {
                let child = node.make_decision(decision);
                best = max(best, aux::<G, _>(&child, depth - 1, false, eval));
            }
            return best;
        } else {
            let mut best = i32::MAX;
            for decision in node.decisions(maximizing) {
                let child = node.make_decision(decision);
                best = min(best, aux::<G, _>(&child, depth - 1, true, eval));
            }
            return best;
        }
    }

    if maximizing {
        node.decisions(maximizing)
            .max_by_key(|&decision| {
                aux::<G, _>(&node.make_decision(decision), depth, false, eval)
            })
    } else {
        node.decisions(maximizing)
            .min_by_key(|&decision| {
                aux::<G, _>(&node.make_decision(decision), depth, true, eval)
            })
    }
}

pub fn alpha_beta_pruning<G: Game, F>(node: &G::State, depth: usize, maximizing: bool, eval: F)
    -> Option<<G::State as Node<G>>::Decision>
where
    F: Fn(&G::State) -> i32 + Copy
{
    fn aux<G: Game, F>(node: &G::State, depth: usize, mut alpha: i32, mut beta: i32, maximizing: bool, eval: F) -> i32
    where
        F: Fn(&G::State) -> i32 + Copy
    {
        if depth == 0 || node.is_terminal() {
            return eval(node);
        }
        if maximizing {
            let mut best = i32::MIN;
            for decision in node.decisions(maximizing) {
                let child = node.make_decision(decision);
                best = max(best, aux::<G, _>(&child, depth - 1, alpha, beta, false, eval));
                alpha = max(alpha, best);
                if alpha >= beta {
                    break;
                }
            }
            return best;
        } else {
            let mut best = i32::MAX;
            for decision in node.decisions(maximizing) {
                let child = node.make_decision(decision);
                best = min(best, aux::<G, _>(&child, depth - 1, alpha, beta, true, eval));
                beta = min(beta, best);
                if alpha >= beta {
                    break;
                }
            }
            return best;
        }
    }

    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;
    let mut best_decision: Option<<G::State as Node<G>>::Decision> = None;
    if maximizing {
        let mut best = i32::MIN;
        for decision in node.decisions(maximizing) {
            let child = node.make_decision(decision);
            let value = aux::<G, _>(&child, depth, alpha, beta, false, eval);
            if value > best {
                best = value;
                best_decision = Some(decision);
            }
            alpha = max(alpha, best);
            if alpha >= beta {
                break;
            }
        }
        return best_decision;
    } else {
        let mut best = i32::MAX;
        for decision in node.decisions(maximizing) {
            let child = node.make_decision(decision);
            let value = aux::<G, _>(&child, depth, alpha, beta, true, eval);
            if value < best {
                best = value;
                best_decision = Some(decision);
            }
            beta = min(beta, best);
            if alpha >= beta {
                break;
            }
        }
        return best_decision;
    }
}

use crate::connectfour::{ConnectFour, Board};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[wasm_bindgen(js_name = minimaxLineCounter)]
pub fn connect_four_minimax_row_counter(board: &Board, maximizing: bool, depth: usize, singles: i32, doubles: i32, triples: i32, quadruples: i32) -> usize
{
    use crate::connectfour::eval::line_counter;

    minimax::<ConnectFour, _>(
        &board,
        depth,
        maximizing,
        &line_counter(singles, doubles, triples, quadruples)
    ).unwrap().column
}

#[allow(dead_code)]
#[wasm_bindgen(js_name = alphaBetaPruningLineCounter)]
pub fn connect_four_alpha_beta_pruning_row_counter(board: &Board, maximizing: bool, depth: usize, singles: i32, doubles: i32, triples: i32, quadruples: i32) -> usize
{
    use crate::connectfour::eval::line_counter;

    alpha_beta_pruning::<ConnectFour, _>(
        &board,
        depth,
        maximizing,
        &line_counter(singles, doubles, triples, quadruples)
    ).unwrap().column
}
