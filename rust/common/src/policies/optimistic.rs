use crate::{Policy, ScoredState};
use rayon::prelude::*;

pub struct OptimisticPolicy {
    depth: usize,
    parallel_depth: usize,
}

impl OptimisticPolicy {
    /// parallel depth is the depth it is still allowed to run on multiple cores for
    /// 1, 0 is fully sequential
    /// 0, 1 is fully parallel
    pub fn from_depth(parallel_depth: usize, sequential_depth: usize) -> Self {
        Self { parallel_depth, depth: sequential_depth }
    }
}

fn get_max_score<S: ScoredState>(state: &S, depth: usize) -> i32 {
    if depth == 0 {
        return state.score();
    }

    let moves = state.possible_moves();
    moves.as_ref().iter()
        .map(|mv| {
            let new_state = state.apply(mv);
            get_max_score(&new_state, depth - 1)
        })
        .max()
        .unwrap_or_else(|| state.score())
}

fn par_get_max_score<S>(state: &S, depth: usize, parallel_depth: usize) -> i32
where
    S: ScoredState + Sync,
    S::Move: Sync,
{
    if parallel_depth == 0 {
        return get_max_score(state, depth)
    }

    let moves = state.possible_moves();
    moves.as_ref().par_iter()
        .map(|mv| {
            let new_state = state.apply(mv);
            get_max_score(&new_state, depth - 1)
        })
        .max()
        .unwrap_or_else(|| state.score())
}

impl<S> Policy<S> for OptimisticPolicy
where
    S: ScoredState + Sync,
    S::Move: Sync,
{
    fn choose_move<'a>(&mut self, state: &S, moves: &'a [S::Move]) -> &'a S::Move {
        assert!(self.depth + self.parallel_depth > 0, "MinMaxPolicy requires total_depth >= 1");

        if self.parallel_depth == 0 {
            moves.iter().max_by_key(|mv| {
                get_max_score(&state.apply(mv), self.depth - 1)
            }).expect("choose_move expects to have at least one move")
        }

        else {
            moves.par_iter().max_by_key(|mv| {
                par_get_max_score(&state.apply(mv), self.depth, self.parallel_depth - 1)
            }).expect("choose_move expects to have at least one move")
        }
    }
}
