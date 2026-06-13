use wasm_bindgen::prelude::*;
use serde::Serialize;
use crate::simulation;

#[derive(Serialize)]
struct CompactResult {
    seed: u32,
    completed_missions: u32,
    turns: u32,
    victory: bool,
}

#[wasm_bindgen]
pub fn launch_single(random: bool, seed: u64, seq_depth: usize) -> JsValue {
    let res = simulation::launch_single(false, random, seed, 0, seq_depth);
    let compact = CompactResult {
        seed: seed as u32,
        completed_missions: res.final_state.completed_missions,
        turns: res.final_state.turn,
        victory: res.final_state.deck_missions.is_empty() && res.final_state.table_missions.is_empty(),
    };
    serde_wasm_bindgen::to_value(&compact).unwrap()
}

#[wasm_bindgen]
pub fn launch_single_trace(random: bool, seed: u64, seq_depth: usize) -> JsValue {
    let res = simulation::launch_single_trace(random, seed, 0, seq_depth);
    serde_wasm_bindgen::to_value(&res).unwrap()
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}
