use crate::game::{card::{_AsBitmask, Card, CardColor}, constants::*, types::TableCards};

pub fn fn_two_separate_cond<F>(table_cards: &TableCards, fx_cond: F) -> bool
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

pub fn fn_two_barely_split_cond<F>(table_cards: &TableCards, fx_cond: F) -> bool
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

pub fn fn_all_distinct_bits<F>(table_cards: &TableCards, fx_to_bitmask: F) -> bool
where
    F: Fn(&'static Card) -> u8
{
    let all_mask: u8 = table_cards.iter()
        .map(|&card| fx_to_bitmask(card))
        .reduce(|acc, bit| acc | bit)
        .expect("No cards were provided to the all distinct check");
    all_mask.count_ones() as usize == table_cards.len()
}

pub fn fn_three_consecutive_ordered(table_cards: &TableCards) -> bool {
    let values: [i8; N_TABLE_CARDS] = std::array::from_fn(|i| table_cards[i].value.get());
    values.windows(3)
        .any(|w| matches!(
            w,
            [a, b, c] if *a == b+1 && *a == c+2 || *a == b-1 && *a == c-2
        ))
}

pub fn fn_four_consecutive(table_cards: &TableCards) -> bool {
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

pub fn fn_col_sum(table_cards: &TableCards, col: CardColor) -> i8 {
    table_cards.iter().filter(|c| c.color == col).map(|c| c.value.get()).sum()
}
