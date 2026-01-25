use crate::{Policy, State};

pub struct RandomPolicy {
    rng: fastrand::Rng,
}

impl RandomPolicy {
    pub fn from_rng(rng: fastrand::Rng) -> Self {
        Self { rng }
    }

    pub fn from_seed(seed: u64) -> Self {
        let rng = fastrand::Rng::with_seed(seed);
        Self { rng }
    }

    /// clock based seed
    pub fn new() -> Self {
        let rng = fastrand::Rng::new();
        Self { rng }
    }
}

impl<S: State> Policy<S> for RandomPolicy {
    fn choose_move<'a>(&mut self, _state: &S, moves: &'a [S::Move]) -> &'a S::Move {
        self.rng.choice(moves.iter()).expect("choose_move expects to have at least one move")
    }
}
