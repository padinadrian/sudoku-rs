//! Number enum

use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

/// Numbers 1 through 9, the possible values for a tile on the board.
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Number {
    #[default]
    Unknown = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::Unknown {
            write!(f, " ")
        } else {
            write!(f, "{}", *self as u8)
        }
    }
}

impl From<u8> for Number {
    /// Will panic if value is out of range!
    fn from(value: u8) -> Self {
        const LOOKUP: [Number; 10] = [
            Number::Unknown,
            Number::One,
            Number::Two,
            Number::Three,
            Number::Four,
            Number::Five,
            Number::Six,
            Number::Seven,
            Number::Eight,
            Number::Nine,
        ];
        return LOOKUP[value as usize];
    }
}
