use crate::{Policy, ScoredState};

pub struct MinMaxPolicy {
    depth: usize,
}

impl MinMaxPolicy {
    pub fn from_depth(depth: usize) -> Self {
        Self { depth }
    }
}

impl<S: ScoredState> Policy<S> for MinMaxPolicy {
    fn choose_move<'a>(&mut self, state: &S, moves: &'a [S::Move]) -> &'a S::Move {
        assert!(self.depth > 0, "MinMaxPolicy requires depth >= 1");

        if self.depth > 1 {
            todo!("Implement deeper MinMaxPolicy");
        }

        moves.iter().max_by_key(|mv| {
            state.apply(mv).score();
        }).expect("choose_move expects to have at least one move")
    }
}
