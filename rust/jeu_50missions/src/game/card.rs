use std::num::NonZeroI8;
use serde::{Serialize, Deserialize, Serializer};

use crate::game::constants::{N_CARDS, N_COLORS, N_VALUES};

const _: () = assert!(ALL_CARDS.len() == 56); // 2*7*4 = 56

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum CardColor {
    Red,
    Green,
    Yellow,
    Blue,
}

pub type CardValue = NonZeroI8;

pub trait _AsBitmask {
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

pub const ALL_CARD_COLORS: &[CardColor] = &[
    CardColor::Blue,
    CardColor::Green,
    CardColor::Red,
    CardColor::Yellow,
];

pub const ALL_CARD_VALUES: &[CardValue] = &[
    unsafe { CardValue::new_unchecked(1) },
    unsafe { CardValue::new_unchecked(2) },
    unsafe { CardValue::new_unchecked(3) },
    unsafe { CardValue::new_unchecked(4) },
    unsafe { CardValue::new_unchecked(5) },
    unsafe { CardValue::new_unchecked(6) },
    unsafe { CardValue::new_unchecked(7) },
];

const fn generate_all_cards() -> [Card; N_CARDS] {
    let mut cards = [Card{
        color: CardColor::Red,
        value: unsafe { CardValue::new_unchecked(1) },
    }; N_CARDS];

    let mut idx_card: usize = 0;
    let mut idx_col = 0;
    while idx_col < N_COLORS {
        let col = ALL_CARD_COLORS[idx_col];

        let mut idx_val = 0;
        while idx_val < N_VALUES {
            let val = ALL_CARD_VALUES[idx_val];

            cards[idx_card] = Card{
                color: col,
                value: val,
            };
            cards[idx_card+1] = cards[idx_card];

            idx_card += 2;
            idx_val += 1;
        }

        idx_col += 1;
    }

    cards
}

pub const ALL_CARDS: [Card; N_CARDS] = generate_all_cards();

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Card {
    pub color: CardColor,
    pub value: CardValue,
}

#[derive(Copy, Clone)]
pub struct CardRef(pub &'static Card);

impl Serialize for CardRef {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

