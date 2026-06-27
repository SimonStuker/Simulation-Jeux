mod game;

mod simulation;
mod dumper;

use std::path::PathBuf;
use clap::Parser;
use indicatif::ProgressIterator;
use crate::{dumper::write_results, simulation::{SimulationResult, launch_single}};

#[derive(Parser)]
#[command(name = "50missions", about = "50 Missions simulation runner")]
struct Args {
    #[arg(short, long, help = "Use random policy (default: optimistic)")]
    random: bool,

    #[arg(short = 'q', long, help = "Print each game state")]
    verbose: bool,

    #[arg(short, long, default_value_t = 1, help = "Number of games to run")]
    batch_size: usize,

    #[arg(short, long, help = "Starting seed (default: current time)")]
    seed: Option<u64>,

    #[arg(short, long, help = "Save results to file")]
    output: Option<PathBuf>,

    #[arg(long, default_value_t = 0, help = "Parallel lookahead depth")]
    par_depth: usize,

    #[arg(long, default_value_t = 1, help = "Sequential lookahead depth")]
    seq_depth: usize,
}

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
    let res_quantiles: Vec<&SimulationResult> = quantiles.iter().map(|&q| get_quantile(results, q)).collect();

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
    if std::env::args().len() == 1 {
        use clap::CommandFactory;
        Args::command().print_help().unwrap();
        println!();
        return;
    }

    let args = Args::parse();

    let seed = args.seed.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    });

    let policy_name = if args.random {
        "RandomPolicy".to_string()
    } else {
        let par_str = if args.par_depth > 0 { format!("parallel_depth={}, ", args.par_depth) } else { String::new() };
        format!("OptimisticPolicy {{ {}sequential_depth={} }}", par_str, args.seq_depth)
    };

    println!("Launching [{}] batches with policy [{}]", args.batch_size, policy_name);
    let mut results = launch_batch(args.verbose, args.random, args.batch_size, seed, args.par_depth, args.seq_depth);
    results.sort_unstable_by_key(|res| res.final_state.completed_missions);

    if let Some(output) = args.output {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create output directory");
        }
        write_results(&results, &output).expect("Failed to write results to file");
        println!("Simulation completed, results saved to {}", output.display());
    } else {
        announce_results(&results, args.verbose);
    }
}
