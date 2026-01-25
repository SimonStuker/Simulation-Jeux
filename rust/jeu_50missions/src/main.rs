use crate::game::{state::State};
use indicatif::{ProgressIterator};

mod game;

fn launch_single(quiet: bool, random: bool, seed: u64) -> (State, State) {
    let mut rng = fastrand::Rng::with_seed(seed);
    let initial_state = State::from_rng(&mut rng);

    let final_state = {
        if random {
            let mut policy = common::policies::RandomPolicy::from_rng(rng);
            common::run_simulation(initial_state.clone(), &mut policy, move |s| {
                if !quiet {
                    s.print_state();
                }
            })
        }

        else {
            let mut policy = common::policies::OptimisticPolicy::from_depth(3, 6);
            common::run_simulation(initial_state.clone(), &mut policy, move |s| {
                if !quiet {
                    s.print_state();
                }
            })
        }
    };

    (initial_state, final_state)
}

fn launch_batch(quiet: bool, random: bool, batch_size: usize, initial_seed: u64) -> Vec<(State, State)> {
    let mut start_stop_states: Vec<(State, State)> = Vec::new();

    for batch_idx in (0..batch_size).progress() {
        let seed = initial_seed.wrapping_add(batch_idx as u64);
        start_stop_states.push(launch_single(quiet, random, seed));
    }

    start_stop_states
}

/// expects sorted data
fn quantile_u32(sorted_data: &mut [u32], q: f64) -> u32 {
    assert!(!sorted_data.is_empty());
    assert!((0.0..=1.0).contains(&q));
    let idx = ((sorted_data.len() - 1) as f64 * q).round() as usize;
    sorted_data[idx]
}

fn main() {
    // Parse command line arguments
    let quiet = std::env::args().any(|arg| arg == "-q" || arg == "--quiet");
    let random = std::env::args().any(|arg| arg == "-r" || arg == "--random");
    let batch_size = std::env::args().zip(std::env::args().skip(1))
        .find(|(arg, _)| arg == "-b" || arg == "--batch_size")
        .map(|(_, val)| val.parse::<usize>().expect("Batch size must be a positive integer"))
        .unwrap_or(1);
    let seed = std::env::args().zip(std::env::args().skip(1))
        .find(|(arg, _)| arg == "-s" || arg == "--seed")
        .map(|(_, val)| val.parse::<u64>().expect("Seed must be a positive integer"))
        .unwrap_or_else(|| {
            let now = std::time::SystemTime::now();
            now.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards").as_millis() as u64
        });

    let results = launch_batch(quiet, random, batch_size, seed);
    let mut missions_completed: Vec<u32> = results.iter().map(|(_, final_state)| final_state.completed_missions).collect();
    missions_completed.sort_unstable();
    let quantiles = [0.0, 0.1, 0.5, 0.9, 0.99, 1.0];
    let mission_quantiles: Vec<u32> = quantiles.iter().map(|&q| quantile_u32(&mut missions_completed, q)).collect();

    for (batch_idx, (initial_state, final_state)) in results.iter().enumerate() {
        println!("=== Game {} : {} completed missions", batch_idx + 1, final_state.completed_missions);
        if !quiet {
            println!("Initial State:");
            initial_state.print_state();
            println!("Final State:");
            final_state.print_state();
        }
    }

    for (q, val) in quantiles.iter().zip(mission_quantiles.iter()) {
        println!("Quantile {:.2} : {} completed missions", q, val);
    }
}
