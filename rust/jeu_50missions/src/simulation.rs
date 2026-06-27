use crate::game::state::{Move, State};
use serde::{Serialize};


#[derive(Serialize, Clone)]
pub struct SimulationResult {
    pub seed: u64,
    pub initial_state: State,
    pub final_state: State,
}

#[derive(Serialize, Clone)]
pub struct SimulationTrace {
    pub seed: u64,
    pub state_list: Vec<State>,
    pub move_list: Vec<Move>,
}

pub fn launch_single(verbose: bool, random: bool, seed: u64, par_depth: usize, seq_depth: usize) -> SimulationResult {
    let mut rng = fastrand::Rng::with_seed(seed);
    let initial_state = State::from_rng(&mut rng);

    let final_state = {
        if random {
            let mut policy = common::policies::RandomPolicy::from_rng(rng);
            common::run_simulation(initial_state.clone(), &mut policy, move |s, _m| {
                if verbose {
                    s.print_state();
                }
            })
        }

        else {
            let mut policy = common::policies::OptimisticPolicy::from_depth(par_depth, seq_depth);
            common::run_simulation(initial_state.clone(), &mut policy, move |s, _m| {
                if verbose {
                    s.print_state();
                }
            })
        }
    };

    SimulationResult { seed, initial_state, final_state }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn launch_single_trace(verbose: bool, random: bool, seed: u64, par_depth: usize, seq_depth: usize) -> SimulationTrace {
    let mut rng = fastrand::Rng::with_seed(seed as u64);
    let initial_state = crate::game::state::State::from_rng(&mut rng);

    let mut state_list = Vec::new();
    let mut move_list = Vec::new();

    if random {
        let mut policy = common::policies::RandomPolicy::from_rng(rng);

        common::run_simulation(initial_state, &mut policy, |state, maybe_mov| {
            if verbose {
                state.print_state();
            }
            state_list.push(state.clone());
            if let Some(mov) = maybe_mov {
                move_list.push(mov.clone());
            }
        })
    } else {
        let mut policy = common::policies::OptimisticPolicy::from_depth(par_depth, seq_depth.max(1) as usize);

        common::run_simulation(initial_state, &mut policy, |state, maybe_mov| {
            if verbose {
                state.print_state();
            }
            state_list.push(state.clone());
            if let Some(mov) = maybe_mov {
                move_list.push(mov.clone());
            }
        })
    };

    return SimulationTrace { seed, state_list, move_list };
}
