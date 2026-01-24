use std::{num::NonZeroU8};
use smallvec::{smallvec, SmallVec};

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

// sanity checks (compile time)
const _: () = assert!(ALL_MISSIONS.len() == 50); // it's called 50 missions for a reason :D
const _: () = assert!(ALL_CARDS.len() == 56); // 2*7*4 = 56
const _: () = assert!(N_MISSIONS <= N_MAX_MISSIONS);
const _: () = assert!(N_CARDS <= N_MAX_CARDS);

#[derive(Clone)]
struct Mission {
    name: &'static str,
    constraint: fn(&TableCards) -> bool,
}

fn two_separate_cond<F>(table_cards: &TableCards, fx_cond: F) -> bool
where
    F: Fn(&'static Card) -> bool
{
    let matching_cond: [bool; N_TABLE_CARDS] = std::array::from_fn(|i| fx_cond(table_cards[i]) );
    let two_adjacent = matching_cond.iter()
        .zip(matching_cond.iter().skip(1))
        .all(|(&cond1, &cond2)| cond1 && cond2);
    let at_least_two = matching_cond.iter().filter(|&&cond| cond).count() >= 2;
    at_least_two && !two_adjacent
}

fn two_barely_split_cond<F>(table_cards: &TableCards, fx_cond: F) -> bool
where
    F: Fn(&'static Card) -> bool
{
    let matching_cond: [bool; N_TABLE_CARDS] = std::array::from_fn(|i| fx_cond(table_cards[i]) );
    let two_are_barely_split = matching_cond.iter()
        .zip(matching_cond.iter().skip(2))
        .all(|(&cond1, &cond2)| cond1 && cond2);
    let exactly_two = matching_cond.iter().filter(|&&cond| cond).count() == 2;
    exactly_two && two_are_barely_split
}

fn all_distinct_bits<F>(table_cards: &TableCards, fx_to_bitmask: F) -> bool
where
    F: Fn(&'static Card) -> u8
{
    let all_mask: u8 = table_cards.iter()
        .map(|&card| fx_to_bitmask(card))
        .reduce(|acc, bit| acc | bit)
        .expect("No cards were provided to the all distinct check");
    all_mask.count_ones() as usize == table_cards.len()
}

fn fn_three_consecutive_ordered(table_cards: &TableCards) -> bool {
    let values: [u8; N_TABLE_CARDS] = std::array::from_fn(|i| table_cards[i].value.get());
    values.windows(3)
        .any(|w| matches!(
            w,
            [a, b, c] if *a == b+1 && *a == c+2 || *a == b-1 && *a == c-2
        ))
}

fn fn_four_consecutive(table_cards: &TableCards) -> bool {
    let mask = table_cards.iter()
        .map(|c| c.value.as_bitmask())
        .reduce(|acc, bit| acc | bit)
        .expect("No cards were provided to the four consecutive check");
    for i in 0..=4 {
        if (mask >> i) & 0xF == 0xF {
            return true;
        }
    }
    false
}

fn fn_col_sum(table_cards: &TableCards, col: CardColor) -> u8 {
    table_cards.iter().filter(|c| c.color == col).map(|c| c.value.get()).sum()
}

macro_rules! define_missions {
    (
        $mission_ident:ident = {
            $(
                $name:ident => |$arg:pat_param| $body:expr
            ),+ $(,)?
        }
    ) => {
        $(
            fn $name($arg: &TableCards) -> bool {
                $body
           }
        )+

        const $mission_ident: &[Mission] = &[
            $(
                Mission {
                    name: stringify!($name),
                    constraint: $name,
                }
            ),+
        ];
    };
}

define_missions! {
    ALL_MISSIONS = {
        sum_10 => |t| t.iter().map(|c| c.value.get()).sum::<u8>() == 10u8,
        sum_15 => |t| t.iter().map(|c| c.value.get()).sum::<u8>() == 15u8,
        sum_18 => |t| t.iter().map(|c| c.value.get()).sum::<u8>() == 18u8,
        sum_20 => |t| t.iter().map(|c| c.value.get()).sum::<u8>() == 20u8,

        all_red_or_blue     => |t| t.iter().all(|c| matches!(c.color, CardColor::Red      | CardColor::Blue  )),
        all_yellow_or_blue  => |t| t.iter().all(|c| matches!(c.color, CardColor::Yellow   | CardColor::Blue  )),
        all_red_or_green    => |t| t.iter().all(|c| matches!(c.color, CardColor::Red      | CardColor::Green )),
        all_yellow_or_green => |t| t.iter().all(|c| matches!(c.color, CardColor::Yellow   | CardColor::Green )),

        red_sum_4     => |t| t.iter().filter(|c| c.color == CardColor::Red   ).map(|c| c.value.get()).sum::<u8>() ==  4u8,
        red_sum_10    => |t| t.iter().filter(|c| c.color == CardColor::Red   ).map(|c| c.value.get()).sum::<u8>() == 10u8,
        yellow_sum_2  => |t| t.iter().filter(|c| c.color == CardColor::Yellow).map(|c| c.value.get()).sum::<u8>() ==  2u8,
        yellow_sum_11 => |t| t.iter().filter(|c| c.color == CardColor::Yellow).map(|c| c.value.get()).sum::<u8>() == 11u8,
        blue_sum_3    => |t| t.iter().filter(|c| c.color == CardColor::Blue  ).map(|c| c.value.get()).sum::<u8>() ==  3u8,
        blue_sum_9    => |t| t.iter().filter(|c| c.color == CardColor::Blue  ).map(|c| c.value.get()).sum::<u8>() ==  9u8,
        green_sum_6   => |t| t.iter().filter(|c| c.color == CardColor::Green ).map(|c| c.value.get()).sum::<u8>() ==  6u8,
        green_sum_7   => |t| t.iter().filter(|c| c.color == CardColor::Green ).map(|c| c.value.get()).sum::<u8>() ==  7u8,

        all_odd  => |t| t.iter().all(|c| c.value.get() % 2u8 == 1u8),
        all_even => |t| t.iter().all(|c| c.value.get() % 2u8 == 0u8),

        all_greater_than_5 => |t| t.iter().all(|c| c.value.get() >= 5u8),
        all_lower_then_3   => |t| t.iter().all(|c| c.value.get() <= 3u8),

        three_red    => |t| t.iter().filter(|c| c.color == CardColor::Red   ).count() == 3,
        three_yellow => |t| t.iter().filter(|c| c.color == CardColor::Yellow).count() == 3,
        three_blue   => |t| t.iter().filter(|c| c.color == CardColor::Blue  ).count() == 3,
        three_green  => |t| t.iter().filter(|c| c.color == CardColor::Green ).count() == 3,

        two_adjacent_red    => |t| t.iter().zip(t.iter().skip(1)).any(|(c1, c2)| c1.color == CardColor::Red    && c2.color == CardColor::Red   ),
        two_adjacent_yellow => |t| t.iter().zip(t.iter().skip(1)).any(|(c1, c2)| c1.color == CardColor::Yellow && c2.color == CardColor::Yellow),
        two_adjacent_blue   => |t| t.iter().zip(t.iter().skip(1)).any(|(c1, c2)| c1.color == CardColor::Blue   && c2.color == CardColor::Blue  ),
        two_adjacent_green  => |t| t.iter().zip(t.iter().skip(1)).any(|(c1, c2)| c1.color == CardColor::Green  && c2.color == CardColor::Green ),

        two_separate_red     => |t| two_separate_cond(t, |c| c.color == CardColor::Red   ),
        two_separate_yellow  => |t| two_separate_cond(t, |c| c.color == CardColor::Yellow),
        two_separate_blue    => |t| two_separate_cond(t, |c| c.color == CardColor::Blue  ),
        two_separate_green   => |t| two_separate_cond(t, |c| c.color == CardColor::Green ),
        two_separate_odds    => |t| two_separate_cond(t, |c| c.value.get() % 2 == 1      ),

        two_barely_split_red     => |t| two_barely_split_cond(t, |c| c.color == CardColor::Red   ),
        two_barely_split_yellow  => |t| two_barely_split_cond(t, |c| c.color == CardColor::Yellow),
        two_barely_split_blue    => |t| two_barely_split_cond(t, |c| c.color == CardColor::Blue  ),
        two_barely_split_green   => |t| two_barely_split_cond(t, |c| c.color == CardColor::Green ),

        all_distinct_values            => |t| all_distinct_bits(t, |c| c.value.as_bitmask()),
        all_distinct_colors            => |t| all_distinct_bits(t, |c| c.color.as_bitmask()),
        all_distinct_colors_and_values => |t| all_distinct_bits(t, |c| c.value.as_bitmask()) && all_distinct_bits(t, |c| c.color.as_bitmask()),

        three_consecutive_ordered => |t| fn_three_consecutive_ordered(t),
        four_consecutive          => |t| fn_four_consecutive(t),

        sum_yellow_equals_green => |t| fn_col_sum(t, CardColor::Yellow) == fn_col_sum(t, CardColor::Green),
        sum_yellow_equals_red   => |t| fn_col_sum(t, CardColor::Yellow) == fn_col_sum(t, CardColor::Red  ),
        sum_blue_equals_green   => |t| fn_col_sum(t, CardColor::Blue  ) == fn_col_sum(t, CardColor::Green),
        sum_blue_equals_red     => |t| fn_col_sum(t, CardColor::Blue  ) == fn_col_sum(t, CardColor::Red  ),

        twice_sum_yellow_equals_green => |t| 2u8 * fn_col_sum(t, CardColor::Yellow) == fn_col_sum(t, CardColor::Green),
        twice_sum_yellow_equals_red   => |t| 2u8 * fn_col_sum(t, CardColor::Yellow) == fn_col_sum(t, CardColor::Red  ),
        twice_sum_blue_equals_green   => |t| 2u8 * fn_col_sum(t, CardColor::Blue  ) == fn_col_sum(t, CardColor::Green),
        twice_sum_blue_equals_red     => |t| 2u8 * fn_col_sum(t, CardColor::Blue  ) == fn_col_sum(t, CardColor::Red  ),
    }
}

const ALL_CARD_COLORS: &[CardColor] = &[
    CardColor::Blue,
    CardColor::Green,
    CardColor::Red,
    CardColor::Yellow,
];

const ALL_CARD_VALUES: &[CardValue] = &[
    unsafe { CardValue::new_unchecked(1) },
    unsafe { CardValue::new_unchecked(2) },
    unsafe { CardValue::new_unchecked(3) },
    unsafe { CardValue::new_unchecked(4) },
    unsafe { CardValue::new_unchecked(5) },
    unsafe { CardValue::new_unchecked(6) },
    unsafe { CardValue::new_unchecked(7) },
];

const ALL_CARDS: [Card; N_CARDS] = {
    let mut cards = [Card{
        color: CardColor::Red,
        value: unsafe { CardValue::new_unchecked(1) },
    }; N_CARDS];

    let mut idx: usize = 0;
    let mut idx_col = 0;
    let mut idx_val = 0;
    while idx_col < N_COLORS {
        let col = ALL_CARD_COLORS[idx_col];

        while idx_val < N_VALUES {
            let val = ALL_CARD_VALUES[idx_val];

            cards[idx] = Card{
                color: col,
                value: val,
            };
            cards[idx+1] = cards[idx];

            idx += 1;
            idx_val += 2;
        }

        idx_col += 1;
    }

    cards
};

#[derive(PartialEq, Clone, Copy)]
enum CardColor {
    Red,
    Green,
    Yellow,
    Blue,
}

type CardValue = NonZeroU8;

trait _AsBitmask {
    fn as_bitmask(self) -> u8;
}

impl _AsBitmask for CardColor {
    fn as_bitmask(self) -> u8 {
        match self {
            CardColor::Red => 1u8,
            CardColor::Green => 2u8,
            CardColor::Yellow => 4u8,
            CardColor::Blue => 8u8,
        }
    }
}

impl _AsBitmask for CardValue {
    fn as_bitmask(self) -> u8 {
        1u8 << (self.get() - 1)
    }
}

#[derive(Clone, Copy)]
struct Card {
    color: CardColor,
    value: CardValue,
}

type PlayerHand = SmallVec<[&'static Card; N_HAND_CARDS]>;
type TableCards = SmallVec<[&'static Card; N_TABLE_CARDS]>;
type TableMissions = SmallVec<[&'static Mission; N_TABLE_MISSIONS]>;
type DeckCards = SmallVec<[&'static Card; N_MAX_CARDS]>;
type DeckMissions = SmallVec<[&'static Mission; N_MAX_MISSIONS]>;

#[derive(Clone)]
enum Player {
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

struct Mov {
    idx_hand: usize,
    idx_table: usize,
}

#[derive(Clone)]
struct State {
    current_player: Player,
    player_hands: [PlayerHand; 2],
    table_cards: TableCards,
    table_missions: TableMissions,
    deck_cards: DeckCards,
    deck_missions: DeckMissions,
    turn: u32,
}

fn pop_n_iter<'a, T, const N: usize>(v: &'a mut SmallVec<[T; N]>, n_poped: usize) -> smallvec::Drain<'a, [T; N]>
where
    [T; N]: smallvec::Array
{
    let new_len = v.len().saturating_sub(n_poped);
    v.drain(new_len..)
}

impl State {
    fn from_rng(rng: &mut fastrand::Rng) -> Self {
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
                |mission| !(mission.constraint)(&self.table_cards)
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

fn print_state(state: &State) {
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

    let title = format!(" TURN {} ", state.turn);
    let bar_width = std::cmp::max(40, title.len() + 10);
    let bar = "═".repeat(bar_width);
    println!("╔{}╗", bar);
    println!("║{:^width$}║", title, width = bar_width);
    println!("╚{}╝", bar);

    println!("┌─ Player hands");
    println!("│   Player 0 ( {} cards )", state.player_hands[0].len());
    println!("│     {}", fmt_cards(&state.player_hands[0]));
    println!("│   Player 1 ( {} cards )", state.player_hands[1].len());
    println!("│     {}", fmt_cards(&state.player_hands[1]));
    println!("└────────────────────────");

    println!("┌─ Table cards ( {} cards )", state.table_cards.len());
    println!("│     {}", fmt_cards(&state.table_cards));
    println!("└────────────────────────");

    println!("┌─ Table missions");
    for mission in state.table_missions.iter() {
        println!("│    - {}", mission.name);
    }
    println!("└────────────────────────");

    println!("┌─ Decks");
    println!("│   Cards   : {:3} remaining", state.deck_cards.len());
    println!("│   Missions : {:3} remaining", state.deck_missions.len());
    println!("└────────────────────────");
}

fn main() {
    let mut rng = fastrand::Rng::new(); // clock based seed
    // let mut rng = fastrand::Rng::with_seed(42);

    let initial_state = State::from_rng(&mut rng);
    let mut policy = common::policies::RandomPolicy::from_rng(rng);
    common::run_simulation(initial_state, &mut policy, print_state);
}
