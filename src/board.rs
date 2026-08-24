//! Board class

use crate::number::Number;
use fixedbitset::FixedBitSet;
use std::{fmt, vec};

pub const ROW_SIZE: usize = 9;

/// Convert an x/y coordinate to an array index.
fn to_index(x: usize, y: usize) -> usize {
    return x + (y * ROW_SIZE);
}

/// Convert an x/y coordinate to a square index.
///
/// Each sudoku board has 9 squares of 9 tiles each. The squares are numbered
/// starting with the top-left corner and going from left-to-right and then from
/// top-to-bottom, as such:
///
/// 0 | 1 | 2
///
/// 3 | 4 | 5
///
/// 6 | 7 | 8
///
pub fn to_square_index(x: usize, y: usize) -> usize {
    ((x / 3) % 3) + ((y / 3) * 3)
}

/// Represents a single tile on the board which contains the value
/// (set or unset) and the notes of possible values as a bitset.
#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub notes: FixedBitSet,
    pub value: Option<Number>,
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
            notes: FixedBitSet::with_capacity(ROW_SIZE),
            value: None,
        }
    }
}

impl Tile {
    /// Construct a new tile with a given value.
    pub fn new(value: Number) -> Self {
        Self {
            value: Some(value),
            ..Default::default()
        }
    }

    /// Set a note for the given value on this tile.
    pub fn set_note(&mut self, value: Number) {
        self.notes.insert(value as usize - 1);
    }

    /// Clear a note for the given value on this tile.
    pub fn clear_note(&mut self, value: Number) {
        self.notes.remove(value as usize - 1);
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Row {
    pub tiles: [Tile; ROW_SIZE],
}

impl Row {
    /// Construct a new Row from a given list of values.
    pub fn new(values: &[Number; ROW_SIZE]) -> Self {
        Self {
            tiles: values.map(Tile::new),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Board {
    pub rows: [Row; ROW_SIZE],
}

type SerializedRow = [u8; ROW_SIZE];

#[derive(Clone)]
pub struct RowIter<'a> {
    board: &'a Board,
    row: usize,
    index: usize,
}

impl<'a> RowIter<'a> {
    pub fn new(board: &'a Board, row: usize) -> Self {
        assert!(row < ROW_SIZE);
        Self {
            board,
            row,
            index: 0,
        }
    }
}

impl<'a> Iterator for RowIter<'a> {
    type Item = &'a Tile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= ROW_SIZE {
            return None;
        }
        let index = self.index;
        self.index += 1;
        Some(&self.board.rows[self.row].tiles[index])
    }
}

#[derive(Clone)]
pub struct ColumnIter<'a> {
    board: &'a Board,
    column: usize,
    index: usize,
}

impl<'a> ColumnIter<'a> {
    pub fn new(board: &'a Board, column: usize) -> Self {
        assert!(column < ROW_SIZE);
        Self {
            board,
            column,
            index: 0,
        }
    }
}

impl<'a> Iterator for ColumnIter<'a> {
    type Item = &'a Tile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= ROW_SIZE {
            return None;
        }
        let index = self.index;
        self.index += 1;
        Some(&self.board.rows[index].tiles[self.column])
    }
}

#[derive(Debug, Clone)]
pub struct SquareIndexIter {
    x_start: usize,
    y_start: usize,
    index: usize,
}

impl SquareIndexIter {
    /// Construct a new iterator object for the given square.
    /// The square number `n` must be between 0 and 8 inclusive.
    pub fn new(n: usize) -> Self {
        assert!(n < ROW_SIZE);
        let x_start = (n % 3) * 3;
        let y_start = (n / 3) * 3;
        Self {
            x_start,
            y_start,
            index: 0,
        }
    }
}

impl Iterator for SquareIndexIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= ROW_SIZE {
            return None;
        }
        let index = self.index;
        self.index += 1;

        let x = self.x_start + (index % 3);
        let y = self.y_start + (index / 3);

        Some((x, y))
    }
}

#[derive(Clone)]
pub struct SquareIter<'a> {
    board: &'a Board,
    iter: SquareIndexIter,
}

impl<'a> SquareIter<'a> {
    pub fn new(board: &'a Board, n: usize) -> Self {
        assert!(n < ROW_SIZE);
        Self {
            board,
            iter: SquareIndexIter::new(n),
        }
    }
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = &'a Tile;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((x, y)) = self.iter.next() {
            Some(&self.board.rows[y].tiles[x])
        } else {
            None
        }
    }
}

/// Check if the row has exactly one of each sudoku value.
fn row_is_solved<'a, Iter>(mut tiles_iter: Iter) -> bool
where
    Iter: Iterator<Item = &'a Tile>,
    Iter: Clone,
{
    let tiles_copy = tiles_iter.clone();
    let any_blank = tiles_iter.any(|tile| tile.value.is_none());
    if any_blank {
        return false;
    }

    // Unwrap is safe because of the above check.
    let values: Vec<Number> = tiles_copy
        .map(|tile| tile.value.expect("Tile value should not be blank!"))
        .collect();

    values.contains(&Number::One)
        && values.contains(&Number::Two)
        && values.contains(&Number::Three)
        && values.contains(&Number::Four)
        && values.contains(&Number::Five)
        && values.contains(&Number::Six)
        && values.contains(&Number::Seven)
        && values.contains(&Number::Eight)
        && values.contains(&Number::Nine)
}

impl Board {
    /// Get a ref to the tile at the given coordinates.
    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.rows[y].tiles[x]
    }

    /// Get a mutable ref to the tile at the given coordinates.
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.rows[y].tiles[x]
    }

    /// Get value of the tile at the given coordinates, if it exists.
    pub fn get_value(&self, x: usize, y: usize) -> Option<Number> {
        self.get_tile(x, y).value
    }

    /// Get a single row of the board.
    pub fn get_row<'a>(&'a self, n: usize) -> RowIter<'a> {
        assert!(n < ROW_SIZE);
        RowIter::new(self, n)
    }

    /// Get a single column of the board.
    pub fn get_column<'a>(&'a self, n: usize) -> ColumnIter<'a> {
        assert!(n < ROW_SIZE);
        ColumnIter::new(self, n)
    }

    /// Get a 3x3 square of the board.
    /// TODO: Unit tests for this function.
    pub fn get_square<'a>(&'a self, n: usize) -> SquareIter<'a> {
        assert!(n < ROW_SIZE);
        SquareIter::new(self, n)
    }

    /// Set the value of the tile at the given coordinates.
    pub fn set(&mut self, x: usize, y: usize, value: Option<Number>) {
        self.rows[y].tiles[x].value = value;
    }

    /// Load a saved game from a CSV file.
    pub fn load_from_file(&mut self, path: &str) -> color_eyre::Result<()> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .from_path(path)?;

        for (y, result) in reader.deserialize().enumerate() {
            let row: SerializedRow = result?;
            for x in 0..ROW_SIZE {
                let number = match row[x] {
                    n if (n > 0 && n < 10) => Some(Number::from(n)),
                    _ => None,
                };
                self.set(x, y, number);
            }
        }
        Ok(())
    }

    pub fn load_from_str(&mut self, serialized: &str) -> color_eyre::Result<()> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .from_reader(serialized.as_bytes());

        for (y, result) in reader.deserialize().enumerate() {
            let row: SerializedRow = result?;
            for x in 0..ROW_SIZE {
                self.set(x, y, Some(Number::from(row[x])));
            }
        }
        Ok(())
    }

    /// Check if the board is solved and the game is finished.
    pub fn is_solved(&self) -> bool {
        for n in 0..ROW_SIZE {
            if !row_is_solved(self.get_row(n)) {
                return false;
            }
        }
        for n in 0..ROW_SIZE {
            if !row_is_solved(self.get_column(n)) {
                return false;
            }
        }
        for n in 0..ROW_SIZE {
            if !row_is_solved(self.get_square(n)) {
                return false;
            }
        }
        true
    }

    /// Fill every unsolved space with notes for every value.
    pub fn fill_all_notes(&mut self) {
        for y in 0..ROW_SIZE {
            for x in 0..ROW_SIZE {
                let tile = self.get_tile_mut(x, y);
                if tile.value.is_none() {
                    tile.notes.insert_range(0..ROW_SIZE);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOLVED_BOARD: &str = "1,2,3,4,5,6,7,8,9
4,5,6,7,8,9,1,2,3
7,8,9,1,2,3,4,5,6
2,3,1,5,6,4,8,9,7
5,6,4,8,9,7,2,3,1
8,9,7,2,3,1,5,6,4
3,1,2,6,4,5,9,7,8
6,4,5,9,7,8,3,1,2
9,7,8,3,1,2,6,4,5";

    const UNSOLVED_BOARD: &str = "1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9";

    #[test]
    fn test_to_square_index() {
        assert_eq!(to_square_index(0, 0), 0);
        assert_eq!(to_square_index(1, 0), 0);
        assert_eq!(to_square_index(2, 0), 0);
        assert_eq!(to_square_index(3, 0), 1);
        assert_eq!(to_square_index(7, 1), 2);
        assert_eq!(to_square_index(0, 1), 0);
        assert_eq!(to_square_index(0, 2), 0);
        assert_eq!(to_square_index(0, 3), 3);
        assert_eq!(to_square_index(3, 3), 4);
        assert_eq!(to_square_index(4, 4), 4);
        assert_eq!(to_square_index(8, 5), 5);
        assert_eq!(to_square_index(2, 8), 6);
        assert_eq!(to_square_index(5, 8), 7);
        assert_eq!(to_square_index(8, 8), 8);
    }

    #[test]
    fn test_square_index_iter() {
        let mut iter = SquareIndexIter::new(4);
        assert_eq!(iter.next(), Some((3, 3)));
        assert_eq!(iter.next(), Some((4, 3)));
        assert_eq!(iter.next(), Some((5, 3)));
        assert_eq!(iter.next(), Some((3, 4)));
        assert_eq!(iter.next(), Some((4, 4)));
        assert_eq!(iter.next(), Some((5, 4)));
        assert_eq!(iter.next(), Some((3, 5)));
        assert_eq!(iter.next(), Some((4, 5)));
        assert_eq!(iter.next(), Some((5, 5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_board_get_value() {
        let mut board = Board::default();
        board.load_from_str(SOLVED_BOARD).unwrap();
        assert_eq!(board.get_value(0, 0), Some(Number::One));
        assert_eq!(board.get_value(1, 1), Some(Number::Five));
        assert_eq!(board.get_value(2, 2), Some(Number::Nine));
        assert_eq!(board.get_value(3, 3), Some(Number::Five));
        assert_eq!(board.get_value(4, 4), Some(Number::Nine));
        assert_eq!(board.get_value(5, 5), Some(Number::One));
        assert_eq!(board.get_value(6, 6), Some(Number::Nine));
        assert_eq!(board.get_value(7, 7), Some(Number::One));
        assert_eq!(board.get_value(8, 8), Some(Number::Five));
    }

    #[test]
    fn test_board_get_row() {
        let mut board = Board::default();
        board.load_from_str(SOLVED_BOARD).unwrap();
        let expected_ints = [2, 3, 1, 5, 6, 4, 8, 9, 7];
        let expected_numbers = expected_ints.map(|n| Number::from(n));
        let mut row_iter = board.get_row(3);
        let tiles: [Number; ROW_SIZE] =
            std::array::from_fn(|_| row_iter.next().unwrap().value.unwrap());
        assert_eq!(tiles, expected_numbers);
    }

    #[test]
    fn test_board_get_column() {
        let mut board = Board::default();
        board.load_from_str(SOLVED_BOARD).unwrap();
        let expected_ints = [5, 8, 2, 6, 9, 3, 4, 7, 1];
        let expected_numbers = expected_ints.map(|n| Number::from(n));
        let mut column_iter = board.get_column(4);
        let tiles: [Number; ROW_SIZE] =
            std::array::from_fn(|_| column_iter.next().unwrap().value.unwrap());
        assert_eq!(tiles, expected_numbers);
    }

    #[test]
    fn test_board_get_square() {
        let mut board = Board::default();
        board.load_from_str(SOLVED_BOARD).unwrap();
        let expected_ints = [6, 4, 5, 9, 7, 8, 3, 1, 2];
        let expected_numbers = expected_ints.map(|n| Number::from(n));
        let mut square_iter = board.get_square(7);
        let tiles: [Number; ROW_SIZE] =
            std::array::from_fn(|_| square_iter.next().unwrap().value.unwrap());
        assert_eq!(tiles, expected_numbers);
    }

    #[test]
    fn test_board_is_solved() {
        let mut board = Board::default();
        board.load_from_str(SOLVED_BOARD).unwrap();
        assert!(board.is_solved());
    }

    #[test]
    fn test_board_is_not_solved() {
        let mut board = Board::default();
        board.load_from_str(UNSOLVED_BOARD).unwrap();
        assert!(!board.is_solved());
    }

    #[test]
    fn test_board_fill_all_notes() {
        let mut board = Board::default();
        board.get_tile_mut(3, 5).value = Some(Number::Five);
        board.fill_all_notes();
        assert_eq!(board.get_value(3, 5), Some(Number::Five));
        assert_eq!(board.get_value(3, 4), None);
        assert_eq!(board.get_tile(3, 5).notes.count_ones(0..ROW_SIZE), 0);
        assert_eq!(
            board.get_tile(3, 5).notes.count_zeroes(0..ROW_SIZE),
            ROW_SIZE
        );
        assert_eq!(board.get_tile(3, 4).notes.count_ones(0..ROW_SIZE), ROW_SIZE);
        assert_eq!(board.get_tile(3, 4).notes.count_zeroes(0..ROW_SIZE), 0);
    }
}
