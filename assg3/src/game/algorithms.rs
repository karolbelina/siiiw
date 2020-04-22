use super::{Game, Node, Opponent};
use std::i32;
use std::cmp::{min, max};

pub fn minimax<G: Game, F>(node: &G::State, player: G::Player, depth: usize, eval: F)
    -> Option<<G::State as Node<G>>::Decision>
where
    F: Fn(&G::State) -> i32 + Copy
{
    fn aux<G: Game, F>(node: &G::State, player: G::Player, depth: usize, maximizing: bool, eval: F) -> i32
    where
        F: Fn(&G::State) -> i32 + Copy
    {
        if depth == 0 || node.is_terminal() {
            return eval(node);
        }
        if maximizing {
            let mut best = i32::MIN;
            for decision in node.decisions(player) {
                let child = node.make_decision(decision);
                best = max(best, aux::<G, _>(&child, player.opponent(), depth - 1, false, eval));
            }
            return best;
        } else {
            let mut best = i32::MAX;
            for decision in node.decisions(player) {
                let child = node.make_decision(decision);
                best = min(best, aux::<G, _>(&child, player.opponent(), depth - 1, true, eval));
            }
            return best;
        }
    }

    if player == G::maximizing_player() {
        node.decisions(player)
            .max_by_key(|&decision| {
                aux::<G, _>(&node.make_decision(decision), player.opponent(), depth - 1, false, eval)
            })
    } else {
        node.decisions(player)
            .min_by_key(|&decision| {
                aux::<G, _>(&node.make_decision(decision), player.opponent(), depth - 1, true, eval)
            })
    }
}

use crate::connectfour::{ConnectFour, Board, Disc, DiscDrop};
use wasm_bindgen::prelude::*;
use js_sys;

#[allow(dead_code)]
#[wasm_bindgen(js_name = minimax)]
pub fn connect_four_minimax(player: Disc, depth: usize, evalf: js_sys::Function) -> JsValue
{
    let cb = Closure::wrap(Box::new(move |board: &Board| -> Option<DiscDrop> {
        minimax::<ConnectFour, _>(&board, player, depth, |board: &Board| -> i32 {
            js_sys::Function::call1(&evalf, &JsValue::NULL, &board.clone().into()).unwrap().as_f64().unwrap() as i32
        })
    }) as Box<dyn Fn(&Board) -> Option<DiscDrop>>);

    let js_cb = JsValue::from(cb.as_ref());

    Closure::forget(cb);

    js_cb
}
