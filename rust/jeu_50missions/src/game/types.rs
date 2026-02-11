
use smallvec::SmallVec;

use crate::game::{card::CardRef, constants::*, missions::MissionRef};

pub type PlayerHand = SmallVec<[CardRef; N_HAND_CARDS]>;
pub type TableCards = SmallVec<[CardRef; N_TABLE_CARDS]>;
pub type TableMissions = SmallVec<[MissionRef; N_TABLE_MISSIONS]>;
pub type DeckCards = SmallVec<[CardRef; N_MAX_CARDS]>;
pub type DeckMissions = SmallVec<[MissionRef; N_MAX_MISSIONS]>;
