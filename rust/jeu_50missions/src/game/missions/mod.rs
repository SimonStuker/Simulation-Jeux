use serde::{Serialize, Serializer};

use crate::game::types::TableCards;

pub struct Mission {
    pub name: &'static str,
    pub constraint: fn(&TableCards) -> bool,
}

impl Mission {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn is_completed(&self, table_cards: &TableCards) -> bool {
        (self.constraint)(table_cards)
    }
}

impl Serialize for Mission {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.name.serialize(s)
    }
}

#[derive(Copy, Clone)]
pub struct MissionRef(pub &'static Mission);

impl Serialize for MissionRef {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

mod predicates;
pub mod list;
