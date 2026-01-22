use std::num::NonZeroU8;

pub const HAND_CARDS_SIZE: usize = 4;
pub const TABLE_MISSIONS_SIZE: usize = 4;
pub const TABLE_CARDS_SIZE: usize = 4;

struct Mission {
    name: &'static str,
    constraint: fn(&TableCards) -> bool,
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

        pub const $mission_ident: &[Mission] = &[
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

        two_adjacent_red     => |_| todo!("Mission not implemented: two_adjacent_red"),
        two_adjacent_yellow  => |_| todo!("Mission not implemented: two_adjacent_yellow"),
        two_adjacent_blue    => |_| todo!("Mission not implemented: two_adjacent_blue"),
        two_adjacent_green   => |_| todo!("Mission not implemented: two_adjacent_green"),

        two_separate_red     => |_| todo!("Mission not implemented: two_separate_red"),
        two_separate_yellow  => |_| todo!("Mission not implemented: two_separate_yellow"),
        two_separate_blue    => |_| todo!("Mission not implemented: two_separate_blue"),
        two_separate_green   => |_| todo!("Mission not implemented: two_separate_green"),

        two_barely_split_red     => |_| todo!("Mission not implemented: two_barely_split_red"),
        two_barely_split_yellow  => |_| todo!("Mission not implemented: two_barely_split_yellow"),
        two_barely_split_blue    => |_| todo!("Mission not implemented: two_barely_split_blue"),
        two_barely_split_green   => |_| todo!("Mission not implemented: two_barely_split_green"),

        all_distinct_values            => |_| todo!("Mission not implemented: all_distinct_values"),
        all_distinct_colors            => |_| todo!("Mission not implemented: all_distinct_colors"),
        all_distinct_colors_and_values => |_| todo!("Mission not implemented: all_distinct_colors_and_values"),

        three_consecutive_ordered => |_| todo!("Mission not implemented: three_consecutive_ordered"),
        four_consecutive          => |_| todo!("Mission not implemented: four_consecutive"),

        sum_yellow_equals_green => |_| todo!("Mission not implemented: sum_yellow_equals_green"),
        sum_yellow_equals_red   => |_| todo!("Mission not implemented: sum_yellow_equals_red"),
        sum_blue_equals_green   => |_| todo!("Mission not implemented: sum_blue_equals_green"),
        sum_blue_equals_red     => |_| todo!("Mission not implemented: sum_blue_equals_red"),

        twice_sum_yellow_equals_green => |_| todo!("Mission not implemented: twice_sum_yellow_equals_green"),
        twice_sum_yellow_equals_red   => |_| todo!("Mission not implemented: twice_sum_yellow_equals_red"),
        twice_sum_blue_equals_green   => |_| todo!("Mission not implemented: twice_sum_blue_equals_green"),
        twice_sum_blue_equals_red     => |_| todo!("Mission not implemented: twice_sum_blue_equals_red"),
    }
}


#[derive(PartialEq)]
enum CardColor {
    Red,
    Green,
    Yellow,
    Blue,
}

type CardValue = NonZeroU8;

struct Card {
    color: CardColor,
    value: CardValue,
}

type PlayerHand = [Card; HAND_CARDS_SIZE];
type TableCards = [Card; HAND_CARDS_SIZE];
type TableMissions = [Card; HAND_CARDS_SIZE];

enum Player {
    Player0,
    Player1,
}

struct Mov {
    idx_hand: usize,
    idx_table: usize,
}

struct State {
    current_player: Player,
    player_hands: [PlayerHand; 2],
    table_cards: TableCards,
    table_missions: TableMissions,
    deck_cards: TableMissions,
    deck_missions: TableMissions,
}

fn main() {
    // common::run_simulation(initial_state, policy)
}
