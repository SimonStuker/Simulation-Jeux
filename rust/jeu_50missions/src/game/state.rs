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

        State {
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
        }
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
        }

        if !self.final_sprint && self.completed_missions >= N_RESHUFFLE_MISSIONS {
            self.deck_missions = mission_deck_from_rng(&mut self.rng);
            self.deck_missions.retain(|m| {
              self.table_missions.iter().all(|tm| !std::ptr::eq(*tm, *m))
            });
            self.final_sprint = true;
            assert!(self.deck_missions.len() + self.table_missions.len() == N_MISSIONS, "After reshuffle, total missions should still be 50");
        }

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
