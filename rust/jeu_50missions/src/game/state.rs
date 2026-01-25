use smallvec::{SmallVec, smallvec};

use crate::game::{card::{ALL_CARDS, Card, CardColor}, constants::*, missions::list::ALL_MISSIONS, setup::pop_n_iter, types::{DeckCards, DeckMissions, PlayerHand, TableCards, TableMissions}};

pub struct Mov {
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
    current_player: Player,
    player_hands: [PlayerHand; 2],
    table_cards: TableCards,
    table_missions: TableMissions,
    deck_cards: DeckCards,
    deck_missions: DeckMissions,
    turn: u32,
}

impl State {
    pub fn from_rng(rng: &mut fastrand::Rng) -> Self {
        let mut deck_missions: DeckMissions = ALL_MISSIONS.iter().collect();
        rng.shuffle(&mut deck_missions);

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
    type Move = Mov;
    type Moves = SmallVec<[Self::Move; N_MAX_MOVES]>;

    fn is_terminal(&self) -> bool {
        self.is_defeat() || self.is_victory()
    }

    fn possible_moves(&self) -> Self::Moves {
        let mut moves: Self::Moves = smallvec![];

        for (idx_hand, card_hand) in self.current_hand().iter().enumerate() {
            for (idx_table, card_table) in self.table_cards.iter().enumerate() {
                if card_hand.color == card_table.color || card_hand.value == card_table.value {
                    moves.push(Mov {
                        idx_hand,
                        idx_table,
                    });
                }
            }
        }

        moves
    }

    fn apply(&self, _mv: &Self::Move) -> Self {
        todo!("Pure apply not implemented yet")
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

            let n_missing = N_TABLE_MISSIONS.saturating_sub(self.table_missions.len());

            if n_missing == 0 || self.deck_missions.is_empty() {
                break;
            }

            let drawn = pop_n_iter(&mut self.deck_missions, n_missing);
            self.table_missions.extend(drawn);
        }

        self.current_player = self.current_player.other();
        self.turn += 1;
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
      println!("│   Cards   : {:3} remaining", self.deck_cards.len());
      println!("│   Missions : {:3} remaining", self.deck_missions.len());
      println!("└────────────────────────");
  }
}
