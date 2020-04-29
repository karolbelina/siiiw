use super::{Game, Node};
use std::i32;
use std::cmp::{min, max};
use crate::{log, console_log};

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

    use rand::seq::SliceRandom;

    let mut decisions: Vec<<G::State as Node<G>>::Decision> = Vec::new();
    if maximizing {
        let mut best = i32::MIN;
        for decision in node.decisions(maximizing) {
            let child = node.make_decision(decision);
            let value = aux::<G, _>(&child, depth, false, eval);
            if value > best {
                best = value;
                decisions.clear();
            }
            if value >= best {
                decisions.push(decision);
            }
        }
        console_log!("{:?} -> {}", decisions, best);
        return decisions.choose(&mut rand::thread_rng()).cloned();
    } else {
        let mut best = i32::MAX;
        for decision in node.decisions(maximizing) {
            let child = node.make_decision(decision);
            let value = aux::<G, _>(&child, depth, true, eval);
            if value < best {
                best = value;
                decisions.clear();
            }
            if value <= best {
                decisions.push(decision);
            }
        }
        console_log!("{:?} -> {}", decisions, best);
        return decisions.choose(&mut rand::thread_rng()).cloned();
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

    use rand::seq::SliceRandom;

    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;
    let mut decisions: Vec<<G::State as Node<G>>::Decision> = Vec::new();
    if maximizing {
        let mut best = i32::MIN;
        for decision in node.decisions(maximizing) {
            let child = node.make_decision(decision);
            let value = aux::<G, _>(&child, depth, alpha, beta, false, eval);
            if value > best {
                best = value;
                decisions.clear();
            }
            if value >= best {
                decisions.push(decision);
            }
            alpha = max(alpha, best);
            if alpha >= beta {
                break;
            }
        }
        console_log!("{:?} -> {}", decisions, best);
        return decisions.choose(&mut rand::thread_rng()).cloned();
    } else {
        let mut best = i32::MAX;
        for decision in node.decisions(maximizing) {
            let child = node.make_decision(decision);
            let value = aux::<G, _>(&child, depth, alpha, beta, true, eval);
            if value < best {
                best = value;
                decisions.clear();
            }
            if value <= best {
                decisions.push(decision);
            }
            beta = min(beta, best);
            if alpha >= beta {
                break;
            }
        }
        console_log!("{:?} -> {}", decisions, best);
        return decisions.choose(&mut rand::thread_rng()).cloned();
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
