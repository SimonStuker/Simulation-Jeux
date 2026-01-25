use crate::game::{card::{ALL_CARD_COLORS, ALL_CARD_VALUES}, missions::list::ALL_MISSIONS};

pub const N_HAND_CARDS: usize = 4;
pub const N_TABLE_CARDS: usize = 4;
pub const N_TABLE_MISSIONS: usize = 4;
pub const N_MISSIONS: usize = ALL_MISSIONS.len();
pub const N_VALUES: usize = ALL_CARD_VALUES.len();
pub const N_COLORS: usize = ALL_CARD_COLORS.len();
pub const N_CARDS: usize = 2 * N_COLORS * N_VALUES;

// smallvec is only implemented for some sizes
pub const N_MAX_MISSIONS: usize = 64;
pub const N_MAX_CARDS: usize = 64;
pub const N_MAX_MOVES: usize = N_HAND_CARDS * N_TABLE_CARDS; // lucky both are powers of 2

const _: () = assert!(N_MISSIONS <= N_MAX_MISSIONS);
const _: () = assert!(N_CARDS <= N_MAX_CARDS);
