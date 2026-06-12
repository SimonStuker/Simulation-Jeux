use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct SimResult {
    seed: u32,
    completed_missions: u32,
    turns: u32,
    victory: bool,
}

fn run_single(seed: u64, random: bool, seq_depth: usize) -> SimResult {
    let mut rng = fastrand::Rng::with_seed(seed);
    let initial_state = crate::game::state::State::from_rng(&mut rng);

    let final_state = if random {
        let mut policy = common::policies::RandomPolicy::from_rng(rng);
        common::run_simulation(initial_state, &mut policy, |_| {})
    } else {
        let mut policy = common::policies::OptimisticPolicy::from_depth(0, seq_depth.max(1));
        common::run_simulation(initial_state, &mut policy, |_| {})
    };

    let victory = final_state.deck_missions.is_empty() && final_state.table_missions.is_empty();

    SimResult {
        seed: seed as u32,
        completed_missions: final_state.completed_missions,
        turns: final_state.turn,
        victory,
    }
}

#[wasm_bindgen]
pub fn run_batch_wasm(batch_size: u32, initial_seed: u32, random: bool, seq_depth: u32) -> JsValue {
    let mut results: Vec<SimResult> = (0..batch_size)
        .map(|i| run_single(initial_seed.wrapping_add(i) as u64, random, seq_depth as usize))
        .collect();
    results.sort_unstable_by_key(|r| r.completed_missions);
    serde_wasm_bindgen::to_value(&results).unwrap()
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}
