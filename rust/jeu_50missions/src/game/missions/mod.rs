use crate::game::types::TableCards;

#[derive(Clone)]
pub struct Mission {
    name: &'static str,
    constraint: fn(&TableCards) -> bool,
}

impl Mission {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn is_completed(&self, table_cards: &TableCards) -> bool {
        (self.constraint)(table_cards)
    }
}

mod predicates;
pub mod list;
