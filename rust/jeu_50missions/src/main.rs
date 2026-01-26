use crate::game::{state::State};
use indicatif::{ProgressIterator};

mod game;

struct SimulationResult {
    seed: u64,
    initial_state: State,
    final_state: State,
}

fn launch_single(quiet: bool, random: bool, seed: u64) -> SimulationResult {
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
            let mut policy = common::policies::OptimisticPolicy::from_depth(3, 5);
            common::run_simulation(initial_state.clone(), &mut policy, move |s| {
                if !quiet {
                    s.print_state();
                }
            })
        }
    };

    SimulationResult { seed, initial_state, final_state }
}

fn launch_batch(quiet: bool, random: bool, batch_size: usize, initial_seed: u64) -> Vec<SimulationResult> {
    let mut results: Vec<SimulationResult> = Vec::new();

    for batch_idx in (0..batch_size).progress() {
        let seed = initial_seed.wrapping_add(batch_idx as u64);
        results.push(launch_single(quiet, random, seed));
    }

    results
}

/// expects sorted data
fn get_quantile<T>(sorted_data: &[T], q: f64) -> &T {
    assert!(!sorted_data.is_empty());
    assert!((0.0..=1.0).contains(&q));
    let idx = ((sorted_data.len() - 1) as f64 * q).round() as usize;
    &sorted_data[idx]
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

    let mut results = launch_batch(quiet, random, batch_size, seed);
    results.sort_unstable_by_key(|res| res.final_state.completed_missions);

    let quantiles = [0.0, 0.1, 0.5, 0.9, 0.99, 1.0];
    let res_quantiles: Vec<&SimulationResult> = quantiles.iter().map(|&q| get_quantile(&results, q)).collect();

    for (batch_idx, sim_res) in results.iter().enumerate() {
        println!("=== Game {} : {} completed missions", batch_idx + 1, sim_res.final_state.completed_missions);
        if !quiet {
            println!("Seed: {}", sim_res.seed);
            println!("Initial State:");
            sim_res.initial_state.print_state();
            println!("Final State:");
            sim_res.final_state.print_state();
        }
    }

    for (q, res) in quantiles.iter().zip(res_quantiles.iter()) {
        println!("Quantile {:.2} seed: {:10}: {} completed missions", q, res.seed, res.final_state.completed_missions);
    }
}
