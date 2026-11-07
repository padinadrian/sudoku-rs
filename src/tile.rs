//! Tile class
//!
//! Represents a single tile on the game board.

use crate::number::{MAX_NUMBER, Number};
use fixedbitset::FixedBitSet;
use std::fmt;

/// Represents a single tile on the board which contains the value
/// (set or unset) and the notes of possible values as a bitset.
#[derive(Debug, Clone)]
pub struct Tile {
    /// Indicates possible values for this tile
    notes: FixedBitSet,
    /// The current value of this tile
    value: Option<Number>,
    /// If true, this tile's value cannot be changed
    locked: bool,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (self.value, other.value) {
            (Some(value1), Some(value2)) => value1 == value2,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => self.notes == other.notes,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(number) = self.value {
            write!(f, "{}", number as u8)
        } else {
            write!(f, " ")
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            notes: FixedBitSet::with_capacity(MAX_NUMBER),
            value: None,
            locked: false,
        }
    }
}

impl Tile {
    /// Get the current value of the tile, if any.
    pub fn get_value(&self) -> Option<Number> {
        self.value
    }

    /// Set this tile to the given value.
    /// Also clears any notes on this tile.
    pub fn set_value(&mut self, value: Number) {
        if !self.locked {
            self.value = Some(value);
            self.notes.clear();
        }
    }

    /// Set this tile to the given value as u8.
    /// Also clears any notes on this tile.
    pub fn set_value_u8(&mut self, value: u8) {
        self.set_value(Number::from(value));
    }

    /// Clear the value at this tile.
    pub fn clear_value(&mut self) {
        if !self.locked {
            self.value = None;
        }
    }

    /// Check if this tile has a value set.
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Lock this tile so the value cannot be changed.
    pub fn lock(&mut self) {
        self.locked = true;
    }

    /// Unlock this tile so the value can be changed.
    pub fn unlock(&mut self) {
        self.locked = false;
    }

    /// Check if this tile is locked.
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Get the list of notes that are set on this tile.
    pub fn get_notes(&self) -> Vec<Number> {
        self.get_notes_u8()
            .iter()
            .map(|value| Number::from(*value))
            .collect()
    }

    /// Get the list of notes that are set on this tile as u8.
    pub fn get_notes_u8(&self) -> Vec<u8> {
        if self.value.is_some() {
            Vec::new()
        } else {
            self.notes.ones().map(|value| value as u8 + 1).collect()
        }
    }

    /// Set a note for the given value on this tile.
    /// TODO(apadin): Dead code, will be used in gameplay.
    #[allow(dead_code)]
    pub fn set_note(&mut self, value: Number) {
        if self.value.is_none() {
            self.notes.insert(value as usize - 1);
        }
    }

    /// Set a note for the given value on this tile.
    /// TODO(apadin): Dead code, only used for testing.
    #[allow(dead_code)]
    pub fn set_note_u8(&mut self, value: u8) {
        self.set_note(Number::from(value));
    }

    /// Set several notes for the given values on this tile.
    /// TODO(apadin): Dead code, only used for testing.
    #[allow(dead_code)]
    pub fn set_notes(&mut self, values: &[Number]) {
        for value in values {
            self.set_note(*value);
        }
    }

    /// Set several notes for the given values on this tile.
    /// TODO(apadin): Dead code, only used for testing.
    #[allow(dead_code)]
    pub fn set_notes_u8(&mut self, values: &[u8]) {
        for value in values {
            self.set_note_u8(*value);
        }
    }

    pub fn set_all_notes(&mut self) {
        if self.value.is_none() {
            self.notes.insert_range(0..MAX_NUMBER);
        }
    }

    /// Clear a note for the given value on this tile.
    pub fn clear_note(&mut self, value: Number) {
        self.notes.remove(value as usize - 1);
    }

    /// Clear a set of notes for the given values on this tile.
    pub fn clear_notes(&mut self, values: &[Number]) {
        for value in values {
            self.clear_note(*value);
        }
    }

    /// Clear all notes on this tile.
    pub fn clear_all_notes(&mut self) {
        self.notes.clear();
    }

    /// Check if this tile has a note for the given value.
    pub fn has_note(&self, value: Number) -> bool {
        if self.value.is_some() {
            false
        } else {
            self.notes.contains(value as usize - 1)
        }
    }

    /// Set the note if it is not set, or clear it if it is set.
    pub fn toggle_note(&mut self, value: Number) {
        self.notes.toggle(value as usize - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_partial_eq1() {
        let mut tile1 = Tile::default();
        tile1.set_value(Number::Eight);
        let mut tile2 = Tile::default();
        tile2.set_value(Number::Nine);
        assert_eq!(tile1, tile1);
        assert_ne!(tile1, tile2);
    }

    #[test]
    fn test_tile_partial_eq2() {
        let mut tile1 = Tile::default();
        tile1.set_value(Number::Eight);
        let tile2 = Tile::default();
        assert_ne!(tile1, tile2);
        assert_ne!(tile2, tile1);
    }

    #[test]
    fn test_tile_partial_eq3() {
        let mut tile1 = Tile::default();
        let mut tile2 = Tile::default();
        tile1.set_notes_u8(&[1, 2, 3]);
        tile2.set_notes_u8(&[4, 5, 6]);
        assert_eq!(tile1, tile1);
        assert_eq!(tile2, tile2);
        assert_ne!(tile1, tile2);
        assert_ne!(tile2, tile1);
    }

    #[test]
    fn test_tile_has_note() {
        let mut tile = Tile::default();
        tile.set_notes_u8(&[1, 3, 5, 7, 8]);
        assert!(tile.has_note(Number::One));
        assert!(!tile.has_note(Number::Two));
        assert!(tile.has_note(Number::Three));
        assert!(!tile.has_note(Number::Four));
        assert!(tile.has_note(Number::Five));
        assert!(!tile.has_note(Number::Six));
        assert!(tile.has_note(Number::Seven));
        assert!(tile.has_note(Number::Eight));
        assert!(!tile.has_note(Number::Nine));
    }
}
