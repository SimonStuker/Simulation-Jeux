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

#[cfg(test)]
static TEST_EASIEST: Mission = Mission { name: "easiest_mission",   constraint: |_| true  };
#[cfg(test)]
static TEST_IMPOSSIBLE: Mission = Mission { name: "impossible_mission", constraint: |_| false };

#[derive(Copy, Clone)]
pub struct MissionRef(pub u8);

impl MissionRef {
    pub fn mission(self) -> &'static Mission {
        #[cfg(test)]
        match self.0 {
            254 => return &TEST_EASIEST,
            255 => return &TEST_IMPOSSIBLE,
            _ => {}
        }
        &list::ALL_MISSIONS[self.0 as usize]
    }

    pub fn name(self) -> &'static str {
        self.mission().name()
    }

    pub fn is_completed(self, cards: &TableCards) -> bool {
        self.mission().is_completed(cards)
    }
}

impl Serialize for MissionRef {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.mission().serialize(s)
    }
}

mod predicates;
pub mod list;
