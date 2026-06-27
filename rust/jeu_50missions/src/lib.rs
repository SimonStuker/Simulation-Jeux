pub mod game;
pub mod simulation;
pub use game::state::State;

#[cfg(target_arch = "wasm32")]
mod wasm;
