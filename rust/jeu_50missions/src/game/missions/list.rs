use crate::game::missions::MissionRef;
use crate::game::missions::predicates::*;
use crate::game::missions::Mission;
use crate::game::card::CardColor;
use crate::game::types::DeckMissions;
use crate::game::types::TableCards;
use crate::game::card::_AsBitmask;

/// All missions of 50 missions
/// invariants:
/// - mission constraints are pure and rely only on table cards
/// - there are exactly 50 missions

// -- Compile time invariants --
const _: () = assert!(ALL_MISSIONS.len() == 50); // it's called 50 missions for a reason :D

// -- Listing --
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
        sum_10 => |t| t.iter().map(|c| c.0.value.get()).sum::<i8>() == 10i8,
        sum_15 => |t| t.iter().map(|c| c.0.value.get()).sum::<i8>() == 15i8,
        sum_18 => |t| t.iter().map(|c| c.0.value.get()).sum::<i8>() == 18i8,
        sum_20 => |t| t.iter().map(|c| c.0.value.get()).sum::<i8>() == 20i8,

        all_red_or_blue     => |t| t.iter().all(|c| matches!(c.0.color, CardColor::Red      | CardColor::Blue  )),
        all_yellow_or_blue  => |t| t.iter().all(|c| matches!(c.0.color, CardColor::Yellow   | CardColor::Blue  )),
        all_red_or_green    => |t| t.iter().all(|c| matches!(c.0.color, CardColor::Red      | CardColor::Green )),
        all_yellow_or_green => |t| t.iter().all(|c| matches!(c.0.color, CardColor::Yellow   | CardColor::Green )),

        red_sum_4     => |t| t.iter().filter(|c| c.0.color == CardColor::Red   ).map(|c| c.0.value.get()).sum::<i8>() ==  4i8,
        red_sum_10    => |t| t.iter().filter(|c| c.0.color == CardColor::Red   ).map(|c| c.0.value.get()).sum::<i8>() == 10i8,
        yellow_sum_2  => |t| t.iter().filter(|c| c.0.color == CardColor::Yellow).map(|c| c.0.value.get()).sum::<i8>() ==  2i8,
        yellow_sum_11 => |t| t.iter().filter(|c| c.0.color == CardColor::Yellow).map(|c| c.0.value.get()).sum::<i8>() == 11i8,
        blue_sum_3    => |t| t.iter().filter(|c| c.0.color == CardColor::Blue  ).map(|c| c.0.value.get()).sum::<i8>() ==  3i8,
        blue_sum_9    => |t| t.iter().filter(|c| c.0.color == CardColor::Blue  ).map(|c| c.0.value.get()).sum::<i8>() ==  9i8,
        green_sum_6   => |t| t.iter().filter(|c| c.0.color == CardColor::Green ).map(|c| c.0.value.get()).sum::<i8>() ==  6i8,
        green_sum_7   => |t| t.iter().filter(|c| c.0.color == CardColor::Green ).map(|c| c.0.value.get()).sum::<i8>() ==  7i8,

        all_odd  => |t| t.iter().all(|c| c.0.value.get() % 2i8 == 1i8),
        all_even => |t| t.iter().all(|c| c.0.value.get() % 2i8 == 0i8),

        all_greater_than_5 => |t| t.iter().all(|c| c.0.value.get() >= 5i8),
        all_lower_then_3   => |t| t.iter().all(|c| c.0.value.get() <= 3i8),

        three_red    => |t| t.iter().filter(|c| c.0.color == CardColor::Red   ).count() == 3,
        three_yellow => |t| t.iter().filter(|c| c.0.color == CardColor::Yellow).count() == 3,
        three_blue   => |t| t.iter().filter(|c| c.0.color == CardColor::Blue  ).count() == 3,
        three_green  => |t| t.iter().filter(|c| c.0.color == CardColor::Green ).count() == 3,

        two_adjacent_red    => |t| t.iter().zip(t.iter().skip(1)).any(|(c1, c2)| c1.0.color == CardColor::Red    && c2.0.color == CardColor::Red   ),
        two_adjacent_yellow => |t| t.iter().zip(t.iter().skip(1)).any(|(c1, c2)| c1.0.color == CardColor::Yellow && c2.0.color == CardColor::Yellow),
        two_adjacent_blue   => |t| t.iter().zip(t.iter().skip(1)).any(|(c1, c2)| c1.0.color == CardColor::Blue   && c2.0.color == CardColor::Blue  ),
        two_adjacent_green  => |t| t.iter().zip(t.iter().skip(1)).any(|(c1, c2)| c1.0.color == CardColor::Green  && c2.0.color == CardColor::Green ),

        two_separate_red     => |t| fn_two_separate_cond(t, |c| c.color == CardColor::Red   ),
        two_separate_yellow  => |t| fn_two_separate_cond(t, |c| c.color == CardColor::Yellow),
        two_separate_blue    => |t| fn_two_separate_cond(t, |c| c.color == CardColor::Blue  ),
        two_separate_green   => |t| fn_two_separate_cond(t, |c| c.color == CardColor::Green ),
        two_separate_odds    => |t| fn_two_separate_cond(t, |c| c.value.get() % 2 == 1      ),

        two_barely_split_red     => |t| fn_two_barely_split_cond(t, |c| c.color == CardColor::Red   ),
        two_barely_split_yellow  => |t| fn_two_barely_split_cond(t, |c| c.color == CardColor::Yellow),
        two_barely_split_blue    => |t| fn_two_barely_split_cond(t, |c| c.color == CardColor::Blue  ),
        two_barely_split_green   => |t| fn_two_barely_split_cond(t, |c| c.color == CardColor::Green ),

        all_distinct_values            => |t| fn_all_distinct_bits(t, |c| c.value.as_bitmask()),
        all_distinct_colors            => |t| fn_all_distinct_bits(t, |c| c.color.as_bitmask()),
        all_distinct_colors_and_values => |t| fn_all_distinct_bits(t, |c| c.value.as_bitmask()) && fn_all_distinct_bits(t, |c| c.color.as_bitmask()),

        three_consecutive_ordered => |t| fn_three_consecutive_ordered(t),
        four_consecutive          => |t| fn_four_consecutive(t),

        sum_yellow_equals_green => |t| fn_col_sum(t, CardColor::Yellow) == fn_col_sum(t, CardColor::Green),
        sum_yellow_equals_red   => |t| fn_col_sum(t, CardColor::Yellow) == fn_col_sum(t, CardColor::Red  ),
        sum_blue_equals_green   => |t| fn_col_sum(t, CardColor::Blue  ) == fn_col_sum(t, CardColor::Green),
        sum_blue_equals_red     => |t| fn_col_sum(t, CardColor::Blue  ) == fn_col_sum(t, CardColor::Red  ),

        twice_sum_yellow_equals_green => |t| 2i8 * fn_col_sum(t, CardColor::Yellow) == fn_col_sum(t, CardColor::Green),
        twice_sum_yellow_equals_red   => |t| 2i8 * fn_col_sum(t, CardColor::Yellow) == fn_col_sum(t, CardColor::Red  ),
        twice_sum_blue_equals_green   => |t| 2i8 * fn_col_sum(t, CardColor::Blue  ) == fn_col_sum(t, CardColor::Green),
        twice_sum_blue_equals_red     => |t| 2i8 * fn_col_sum(t, CardColor::Blue  ) == fn_col_sum(t, CardColor::Red  ),
    }
}

pub fn mission_deck_from_rng(rng: &mut fastrand::Rng) -> DeckMissions {
    let mut deck_missions: DeckMissions = ALL_MISSIONS.iter().map(|m| MissionRef(m)).collect();
    rng.shuffle(&mut deck_missions);
    deck_missions
}
