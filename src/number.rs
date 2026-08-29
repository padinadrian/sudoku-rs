//! Number enum

use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

/// Numbers 1 through 9, the possible values for a tile on the board.
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Number {
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

pub const ALL_NUMBERS: [Number; 9] = [
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

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl From<u8> for Number {
    /// Will panic if value is out of range!
    fn from(value: u8) -> Self {
        assert!(value >= 1);
        assert!(value <= 9);
        return ALL_NUMBERS[(value - 1) as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_from_int() {
        assert_eq!(Number::from(1), Number::One);
        assert_eq!(Number::from(2), Number::Two);
        assert_eq!(Number::from(3), Number::Three);
        assert_eq!(Number::from(4), Number::Four);
        assert_eq!(Number::from(5), Number::Five);
        assert_eq!(Number::from(6), Number::Six);
        assert_eq!(Number::from(7), Number::Seven);
        assert_eq!(Number::from(8), Number::Eight);
        assert_eq!(Number::from(9), Number::Nine);
    }
}
