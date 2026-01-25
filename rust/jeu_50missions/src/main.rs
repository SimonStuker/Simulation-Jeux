use crate::game::{state::State};

mod game;

fn main() {
    let quiet = std::env::args().any(|arg| arg == "-q" || arg == "--quiet");

    let mut rng = fastrand::Rng::new(); // clock based seed
    // let mut rng = fastrand::Rng::with_seed(42);

    let initial_state = State::from_rng(&mut rng);
    let mut policy = common::policies::RandomPolicy::from_rng(rng);
    common::run_simulation(initial_state, &mut policy, move |s| {
        if !quiet {
            s.print_state();
        }
    });
}
