use crate::game::{state::State};

mod game;

fn main() {
    // Parse command line arguments
    let quiet = std::env::args().any(|arg| arg == "-q" || arg == "--quiet");
    let random = std::env::args().any(|arg| arg == "-r" || arg == "--random");

    // create state
    let mut rng = fastrand::Rng::new(); // clock based seed
    // let mut rng = fastrand::Rng::with_seed(42);
    let initial_state = State::from_rng(&mut rng);

    // strategies
    if random {
        let mut policy = common::policies::RandomPolicy::from_rng(rng);
        common::run_simulation(initial_state, &mut policy, move |s| {
            if !quiet {
                s.print_state();
            }
        });
    }

    else {
        let mut policy = common::policies::OptimisticPolicy::from_depth(3, 7);
        common::run_simulation(initial_state, &mut policy, move |s| {
            if !quiet {
                s.print_state();
            }
        });
    }
}
