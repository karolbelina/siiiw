use super::{Game, Node, NextPlayer};
use std::i32;
use std::cmp::{min, max};

pub fn minimax<G: Game>(node: &G::State, player: G::Player, depth: usize)
    -> Option<<G::State as Node<G>>::Decision>
{
    fn aux<G: Game>(node: G::State, depth: usize, maximizing: G::Player, player: G::Player) -> i32 {
        if depth == 0 {
            return node.evaluate(&player);
        }
        if player == maximizing {
            let mut best = i32::MIN;
            for decision in node.decisions(player) {
                let child = node.make_decision(decision);
                best = max(best, aux::<G>(child, depth - 1, maximizing, player.next_player()))
            }
            return best;
        } else {
            let mut best = i32::MAX;
            for decision in node.decisions(player) {
                let child = node.make_decision(decision);
                best = min(best, aux::<G>(child, depth - 1, maximizing, player.next_player()))
            }
            return best;
        }
    }

    node.decisions(player)
        .max_by_key(|&decision| aux::<G>(node.make_decision(decision), depth, player, player))
}
