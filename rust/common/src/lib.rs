pub mod policies;

pub trait State {
    type Move;
    type Moves: AsRef<[Self::Move]>;

    fn is_terminal(&self) -> bool;
    fn possible_moves(&self) -> Self::Moves;

    fn apply(&self, mv: &Self::Move) -> Self;
    fn apply_mut(&mut self, mv: &Self::Move);
}

pub trait ScoredState: State {
    fn score(&self) -> i32;
}

pub trait Policy<S: State> {
    fn choose_move<'a>(&mut self, state: &S, moves: &'a [S::Move]) -> &'a S::Move;
}

pub fn run_until<S, P, FS, FT>(mut state: S, policy: &mut P, mut fx_stop: FS, mut fx_turn: FT) -> S
where
    S: State,
    P: Policy<S>,
    FS: FnMut(&S) -> bool,
    FT: FnMut(&S, Option<&S::Move>),
{
    while !state.is_terminal() && !fx_stop(&state) {
        let moves = state.possible_moves();
        if moves.as_ref().is_empty() {
            break;
        }
        let chosen_move = policy.choose_move(&state, moves.as_ref());
        fx_turn(&state, Some(&chosen_move));
        state.apply_mut(&chosen_move);
    }

    fx_turn(&state, None);
    state
}

pub fn run_n_steps<S, P, FT>(state: S, policy: &mut P, n: u64, fx_turn: FT) -> S
where
    S: State,
    P: Policy<S>,
    FT: FnMut(&S, Option<&S::Move>),
{
    let mut steps_left = n;
    run_until(state, policy, move |_| {
        if steps_left == 0 {
            return false;
        } else {
            steps_left -= 1;
            return true;
        }
    }, fx_turn)
}

pub fn run_simulation<S, P, FT>(initial_state: S, policy: &mut P, fx_turn: FT) -> S
where
    S: State,
    P: Policy<S>,
    FT: FnMut(&S, Option<&S::Move>),
{
    run_until(initial_state, policy, |_| false, fx_turn)
}
