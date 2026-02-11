// pub player_hands: [PlayerHand; 2],
// pub table_cards: TableCards,
// pub table_missions: TableMissions,
// pub deck_cards: DeckCards,
// pub deck_missions: DeckMissions,
// pub rng: fastrand::Rng,

use crate::game::types::PlayerHand;

impl serde::Serialize for PlayerHand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for card in self.iter() {
            seq.serialize_element(card)?;
        }
        seq.end()
    }
}
