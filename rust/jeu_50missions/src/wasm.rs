use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_simulation_wasm(seed: u32) -> u32 {
    let mut rng = fastrand::Rng::with_seed(seed as u64);
    let initial_state = crate::game::state::State::from_rng(&mut rng);
    let mut policy = common::policies::RandomPolicy::from_rng(rng);
    let final_state = common::run_simulation(initial_state, &mut policy, |_| {});
    final_state.completed_missions
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}
