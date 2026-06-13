mod game;

mod simulation;
mod dumper;

use std::path::PathBuf;
use indicatif::{ProgressIterator};
use crate::{dumper::write_results, simulation::{SimulationResult, launch_single}};

fn get_quantile<T>(sorted_data: &[T], q: f64) -> &T {
    assert!(!sorted_data.is_empty());
    assert!((0.0..=1.0).contains(&q));
    let idx = ((sorted_data.len() - 1) as f64 * q).round() as usize;
    &sorted_data[idx]
}

pub fn launch_batch(verbose: bool, random: bool, batch_size: usize, initial_seed: u64, par_depth: usize, seq_depth: usize) -> Vec<SimulationResult> {
    let mut results: Vec<SimulationResult> = Vec::new();

    for batch_idx in (0..batch_size).progress() {
        let seed = initial_seed.wrapping_add(batch_idx as u64);
        results.push(launch_single(verbose, random, seed, par_depth, seq_depth));
    }

    results
}

fn announce_results(results: &[SimulationResult], verbose: bool) {
    let quantiles = [0.0, 0.1, 0.5, 0.9, 0.99, 1.0];
    let res_quantiles: Vec<&SimulationResult> = quantiles.iter().map(|&q| get_quantile(&results, q)).collect();

    for (batch_idx, sim_res) in results.iter().enumerate() {
        println!("=== Game {} : {} completed missions", batch_idx + 1, sim_res.final_state.completed_missions);
        if verbose {
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

fn main() {
    let verbose = std::env::args().any(|arg| arg == "-q" || arg == "--verbose");
    let random = std::env::args().any(|arg| arg == "-r" || arg == "--random");
    let batch_size = std::env::args().zip(std::env::args().skip(1))
        .find(|(arg, _)| arg == "-b" || arg == "--batch-size")
        .map(|(_, val)| val.parse::<usize>().expect("Batch size must be a positive integer"))
        .unwrap_or(1);
    let seed = std::env::args().zip(std::env::args().skip(1))
        .find(|(arg, _)| arg == "-s" || arg == "--seed")
        .map(|(_, val)| val.parse::<u64>().expect("Seed must be a positive integer"))
        .unwrap_or_else(|| {
            let now = std::time::SystemTime::now();
            now.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards").as_millis() as u64
        });
    let output: Option<PathBuf> = std::env::args().zip(std::env::args().skip(1))
        .find(|(arg, _)| arg == "-o" || arg == "--output")
        .map(|(_, val)| PathBuf::from(val));
    let par_depth = std::env::args().zip(std::env::args().skip(1))
        .find(|(arg, _)| arg == "--par-depth")
        .map(|(_, val)| val.parse::<usize>().expect("Parallel depth must be a non-negative integer"))
        .unwrap_or(0);
    let seq_depth = std::env::args().zip(std::env::args().skip(1))
        .find(|(arg, _)| arg == "--seq-depth")
        .map(|(_, val)| val.parse::<usize>().expect("Sequential depth must be a non-negative integer"))
        .unwrap_or(1);

    if output.is_some() {
        std::fs::create_dir_all(output.as_ref().unwrap().parent().unwrap())
            .expect("Failed to create output directory");
    }

    let policy_name = if random {
        "RandomPolicy".to_string()
    } else {
        let par_str = if par_depth > 0 { format!("parallel_depth={}, ", par_depth) } else { String::new() };
        let seq_str = if seq_depth > 0 { format!("sequential_depth={}", seq_depth) } else { String::new() };
        format!("OptimisticPolicy {{ {}{} }}", par_str, seq_str)
    };

    println!("Launching [{}] batches with policy [{}]", batch_size, policy_name);
    let mut results = launch_batch(verbose, random, batch_size, seed, par_depth, seq_depth);
    results.sort_unstable_by_key(|res| res.final_state.completed_missions);

    if let Some(output) = output {
        write_results(&results, &output).expect("Failed to write results to file");
        println!("Simulation completed, results saved to {}", output.display());
    } else {
        announce_results(&results, verbose);
    }
}
