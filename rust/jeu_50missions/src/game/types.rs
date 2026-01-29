
use smallvec::SmallVec;

use crate::game::{card::Card, constants::*, missions::Mission};

pub type PlayerHand = SmallVec<[&'static Card; N_HAND_CARDS]>;
pub type TableCards = SmallVec<[&'static Card; N_TABLE_CARDS]>;
pub type TableMissions = SmallVec<[&'static Mission; N_TABLE_MISSIONS]>;
pub type DeckCards = SmallVec<[&'static Card; N_MAX_CARDS]>;
pub type DeckMissions = SmallVec<[&'static Mission; N_MAX_MISSIONS]>;
