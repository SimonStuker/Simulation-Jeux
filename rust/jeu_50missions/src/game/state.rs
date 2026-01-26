use common::ScoredState;
use smallvec::{SmallVec, smallvec};

use crate::game::{card::{ALL_CARDS, Card, CardColor}, constants::*, missions::list::mission_deck_from_rng, setup::pop_n_iter, types::{DeckCards, DeckMissions, PlayerHand, TableCards, TableMissions}};

pub struct Move {
    idx_hand: usize,
    idx_table: usize,
}

#[derive(Clone)]
pub enum Player {
    Player0,
    Player1,
}

impl Player {
    fn as_idx(&self) -> usize {
        match self {
            Player::Player0 => 0,
            Player::Player1 => 1,
        }
    }

    fn other(&self) -> Player {
        match self {
            Player::Player0 => Player::Player1,
            Player::Player1 => Player::Player0,
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub current_player: Player,
    pub player_hands: [PlayerHand; 2],
    pub table_cards: TableCards,
    pub table_missions: TableMissions,
    pub deck_cards: DeckCards,
    pub deck_missions: DeckMissions,
    pub turn: u32,
    pub rng: fastrand::Rng,
    pub final_sprint: bool,
    pub completed_missions: u32,
}

impl State {
    pub fn from_rng(rng: &mut fastrand::Rng) -> Self {
        let mut deck_missions: DeckMissions = mission_deck_from_rng(rng);

        let mut deck_cards: DeckCards = ALL_CARDS.iter().collect();
        rng.shuffle(&mut deck_cards);

        let player_hands: [PlayerHand; 2] = [
            pop_n_iter(&mut deck_cards, N_HAND_CARDS).collect(),
            pop_n_iter(&mut deck_cards, N_HAND_CARDS).collect(),
        ];
        let table_cards: TableCards = pop_n_iter(&mut deck_cards, N_TABLE_CARDS).collect();
        let table_missions: TableMissions = pop_n_iter(&mut deck_missions, N_TABLE_MISSIONS).collect();

        assert!(deck_cards.len() + player_hands[0].len() + player_hands[1].len() + table_cards.len() == N_CARDS);
        assert!(deck_missions.len() + table_missions.len() == N_MISSIONS);

        let mut new_state = State {
            current_player: Player::Player0,
            player_hands,
            table_cards,
            table_missions,
            deck_cards,
            deck_missions,
            rng: rng.clone(),
            completed_missions: 0,
            final_sprint: false,
            turn: 0,
        };
        new_state.check_and_complete_missions();
        new_state.check_and_apply_last_sprint();
        new_state
    }

    fn current_hand_mut(&mut self) -> &mut PlayerHand {
        &mut self.player_hands[self.current_player.as_idx()]
    }

    fn current_hand(&self) -> &PlayerHand {
        &self.player_hands[self.current_player.as_idx()]
    }

    fn draw_card(&mut self) -> Option<&'static Card> {
        self.deck_cards.pop()
    }

    fn is_defeat(&self) -> bool {
        self.current_hand().is_empty()
    }

    fn is_victory(&self) -> bool {
        self.deck_missions.is_empty() && self.table_missions.is_empty()
    }

    fn check_and_apply_last_sprint(&mut self) {
        if !self.final_sprint && self.completed_missions >= N_RESHUFFLE_MISSIONS {
            self.deck_missions = mission_deck_from_rng(&mut self.rng);
            self.deck_missions.retain(|m| {
              self.table_missions.iter().all(|tm| !std::ptr::eq(*tm, *m))
            });
            self.final_sprint = true;
            assert!(self.deck_missions.len() + self.table_missions.len() == N_MISSIONS, "After reshuffle, total missions should still be 50");
        }
    }

    /// Note: there is a known potential bug here if you complete more than N_RESHUFFLE_MISSIONS missions at once
    /// This could cause you to go from non-final-sprint to no more missions in the deck, meaning you might have <4 missions on the table
    /// but this is extremely unlikely to happen in practice
    fn check_and_complete_missions(&mut self) {
        loop {
            self.table_missions.retain(
                |mission| !mission.is_completed(&self.table_cards)
            );

            let n_completed = N_TABLE_MISSIONS.saturating_sub(self.table_missions.len());

            if n_completed == 0 || self.deck_missions.is_empty() {
                break;
            }

            self.completed_missions += n_completed as u32;

            let drawn = pop_n_iter(&mut self.deck_missions, n_completed);
            self.table_missions.extend(drawn);

            self.check_and_apply_last_sprint();
        }
    }
}

impl common::State for State {
    type Move = Move;
    type Moves = SmallVec<[Self::Move; N_MAX_MOVES]>;

    fn is_terminal(&self) -> bool {
        self.is_defeat() || self.is_victory()
    }

    fn possible_moves(&self) -> Self::Moves {
        let mut moves: Self::Moves = smallvec![];

        for (idx_hand, card_hand) in self.current_hand().iter().enumerate() {
            for (idx_table, card_table) in self.table_cards.iter().enumerate() {
                if card_hand.color == card_table.color || card_hand.value == card_table.value {
                    moves.push(Move {
                        idx_hand,
                        idx_table,
                    });
                }
            }
        }

        moves
    }

    fn apply(&self, mv: &Self::Move) -> Self {
        let mut new_state = self.clone();
        new_state.apply_mut(mv);
        new_state
    }

    fn apply_mut(&mut self, mv: &Self::Move) {
        let played_card = self.current_hand_mut().swap_remove(mv.idx_hand);
        // careful: order needs to be preserved
        self.table_cards[mv.idx_table] = played_card;

        if let Some(drawn_card) = self.draw_card() {
            self.current_hand_mut().push(drawn_card);
        }

        self.check_and_complete_missions();
        self.check_and_apply_last_sprint();

        self.current_player = self.current_player.other();
        self.turn += 1;
    }
}

impl ScoredState for State {
    fn score(&self) -> i32 {
        self.completed_missions as i32
    }
}

impl State {
  pub fn print_state(self: &Self) {
      fn fmt_card(card: &Card) -> String {
          let color = match card.color {
              CardColor::Red => "Red  ",
              CardColor::Green => "Green",
              CardColor::Yellow => "Yellow",
              CardColor::Blue => "Blue ",
          };
          format!("{}-{}", card.value.get(), color)
      }

      fn fmt_cards(cards: &[&Card]) -> String {
          cards
              .iter()
              .map(|c| fmt_card(c))
              .collect::<Vec<_>>()
              .join(" | ")
      }

      let title = format!(" TURN {} ", self.turn);
      let bar_width = std::cmp::max(40, title.len() + 10);
      let bar = "═".repeat(bar_width);
      println!("╔{}╗", bar);
      println!("║{:^width$}║", title, width = bar_width);
      println!("╚{}╝", bar);

      println!("┌─ Player hands");
      println!("│   Player 0 ( {} cards )", self.player_hands[0].len());
      println!("│     {}", fmt_cards(&self.player_hands[0]));
      println!("│   Player 1 ( {} cards )", self.player_hands[1].len());
      println!("│     {}", fmt_cards(&self.player_hands[1]));
      println!("└────────────────────────");

      println!("┌─ Table cards ( {} cards )", self.table_cards.len());
      println!("│     {}", fmt_cards(&self.table_cards));
      println!("└────────────────────────");

      println!("┌─ Table missions");
      for mission in self.table_missions.iter() {
          println!("│    - {}", mission.name());
      }
      println!("└────────────────────────");

      println!("┌─ Decks");
      println!("│   Cards     : {:3} remaining", self.deck_cards.len());
      println!("│   Missions  : {:3} remaining {:3} completed{}", self.deck_missions.len(), self.completed_missions, if self.final_sprint { " (final sprint)" } else { "" });
      println!("└────────────────────────");
  }
}

#[cfg(test)]
mod tests {
    use common::State;

    use crate::game::missions::{Mission};
    use crate::game::constants::*;

    const TEST_SEED: u64 = 42;

    pub const EASIEST_MISSION: Mission = Mission {
        name: "easiest_mission",
        constraint: |_| true,
    };

    pub const IMPOSSIBLE_MISSION: Mission = Mission {
        name: "impossible_mission",
        constraint: |_| false,
    };

    #[test]
    fn state_initializes() {
        let mut rng = fastrand::Rng::with_seed(42);
        let state = crate::State::from_rng(&mut rng);

        assert_eq!(state.player_hands[0].len(), N_HAND_CARDS);
        assert_eq!(state.player_hands[1].len(), N_HAND_CARDS);
        assert_eq!(state.table_cards.len(), N_TABLE_CARDS);
        assert_eq!(state.table_missions.len(), N_TABLE_MISSIONS);
        assert_eq!(state.deck_cards.len(), N_CARDS - 2 * N_HAND_CARDS - N_TABLE_CARDS);
        assert_eq!(state.deck_missions.len(), N_MISSIONS - N_TABLE_MISSIONS);
        assert_eq!(state.turn, 0);
    }

    #[test]
    fn state_detects_victory() {
        let mut rng = fastrand::Rng::with_seed(TEST_SEED);
        let mut state = crate::State::from_rng(&mut rng);

        assert_eq!(state.completed_missions, 0);
        assert!(!state.is_victory());
        assert!(!state.possible_moves().is_empty());

        state.completed_missions = 49;
        assert!(!state.is_victory());
        assert!(!state.possible_moves().is_empty());

        state.completed_missions = 50;
        assert!(state.is_victory());
        assert!(state.possible_moves().is_empty());
    }

    #[test]
    fn state_detects_defeat_by_no_deck_left() {
        let mut rng = fastrand::Rng::with_seed(TEST_SEED);
        let mut state = crate::State::from_rng(&mut rng);

        state.deck_cards.clear();
        assert!(state.is_defeat());
        assert!(state.possible_moves().is_empty());
    }

    #[test]
    fn state_detects_defeat_by_no_moves_left() {
        let mut rng = fastrand::Rng::with_seed(TEST_SEED);
        let mut state = crate::State::from_rng(&mut rng);

        state.deck_cards.clear();
        assert!(state.is_defeat());
        assert!(state.possible_moves().is_empty());
    }

    #[test]
    fn state_detects_multiple_completed_missions() {
        let mut rng = fastrand::Rng::with_seed(TEST_SEED);
        let mut state = crate::State::from_rng(&mut rng);

        let initial_completed = state.completed_missions;
        let initial_deck_len = state.deck_missions.len();

        assert_eq!(state.table_missions.len(), N_TABLE_MISSIONS);

        // fill deck with impossible missions except for the top 5 easiest ones
        const EXTRA_FROM_DECK: usize = 5;
        state.table_missions.fill(&EASIEST_MISSION);
        state.deck_missions.fill(&IMPOSSIBLE_MISSION);
        state.deck_missions[(initial_deck_len - EXTRA_FROM_DECK)..].fill(&EASIEST_MISSION);

        assert_eq!(state.completed_missions, initial_completed);
        assert!(!state.final_sprint);

        state.check_and_complete_missions();
        state.check_and_apply_last_sprint();
        assert_eq!(state.completed_missions, initial_completed + N_TABLE_MISSIONS as u32 + EXTRA_FROM_DECK as u32, "New missions should also be instantly completed");
        assert!(state.table_missions.iter().all(|m| m.name() == IMPOSSIBLE_MISSION.name()), "All missions on the table should now be impossible missions");
    }
}
