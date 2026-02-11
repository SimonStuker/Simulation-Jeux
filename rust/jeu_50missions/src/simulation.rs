use crate::game::{state::State};
use indicatif::{ProgressIterator};
use serde::{Serialize};


#[derive(Serialize, Clone)]
pub struct SimulationResult {
    pub seed: u64,
    pub initial_state: State,
    pub final_state: State,
}

pub fn launch_single(verbose: bool, random: bool, seed: u64, par_depth: usize, seq_depth: usize) -> SimulationResult {
    let mut rng = fastrand::Rng::with_seed(seed);
    let initial_state = State::from_rng(&mut rng);

    let final_state = {
        if random {
            let mut policy = common::policies::RandomPolicy::from_rng(rng);
            common::run_simulation(initial_state.clone(), &mut policy, move |s| {
                if verbose {
                    s.print_state();
                }
            })
        }

        else {
            let mut policy = common::policies::OptimisticPolicy::from_depth(par_depth, seq_depth);
            common::run_simulation(initial_state.clone(), &mut policy, move |s| {
                if verbose {
                    s.print_state();
                }
            })
        }
    };

    SimulationResult { seed, initial_state, final_state }
}

pub fn launch_batch(verbose: bool, random: bool, batch_size: usize, initial_seed: u64, par_depth: usize, seq_depth: usize) -> Vec<SimulationResult> {
    let mut results: Vec<SimulationResult> = Vec::new();

    for batch_idx in (0..batch_size).progress() {
        let seed = initial_seed.wrapping_add(batch_idx as u64);
        results.push(launch_single(verbose, random, seed, par_depth, seq_depth));
    }

    results
}


