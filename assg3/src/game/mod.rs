mod algorithms;

use std::fmt::Debug;

pub trait Node<G: Game>: Sized {
    type Decision: Copy + Debug;
    type Decisions: Iterator<Item=Self::Decision>;

    fn decisions(&self, player: G::Player) -> Self::Decisions;

    fn make_decision(&self, decision: Self::Decision) -> Self;

    fn is_terminal(&self) -> bool;

    fn evaluate(&self, player: &G::Player) -> i32;
}

pub trait NextPlayer {
    fn next_player(&self) -> Self;
}

pub trait Game: Sized {
    type State: Node<Self>;
    type Player: Copy + Default + NextPlayer + Eq + Debug;
}
