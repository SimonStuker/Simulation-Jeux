pub mod game;
pub use game::state::State;

#[cfg(target_arch = "wasm32")]
mod wasm;
