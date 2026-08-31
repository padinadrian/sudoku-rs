//! Helper functions for solver: Only one choice left

use crate::board::{Board, ROW_SIZE, SquareIndexIter};
use crate::number::ALL_NUMBERS;

/// Resolve tiles that have a note which is the only remaining note of that
/// value in this row.
/// Return after the first updated tile.
fn resolve_only_choice_in_row(board: &mut Board, row: usize) -> bool {
    for value in ALL_NUMBERS {
        // Count the number of tiles with this value in the notes
        let mut found_row = None;
        for column in 0..ROW_SIZE {
            let tile = board.get_tile(column, row);
            let notes = tile.get_notes();
            if notes.contains(&value) {
                if found_row.is_none() {
                    found_row = Some(column);
                } else {
                    found_row = None;
                    break;
                }
            }
        }
        if let Some(column) = found_row {
            board.get_tile_mut(column, row).set_value(value);
            return true;
        }
    }
    false
}

/// Resolve tiles that have a note which is the only remaining note of that
/// value in this column.
/// Return after the first updated tile.
fn resolve_only_choice_in_column(board: &mut Board, column: usize) -> bool {
    for value in ALL_NUMBERS {
        // Count the number of tiles with this value in the notes
        let mut found_row = None;
        for row in 0..ROW_SIZE {
            let tile = board.get_tile(column, row);
            let notes = tile.get_notes();
            if notes.contains(&value) {
                if found_row.is_none() {
                    found_row = Some(row);
                } else {
                    found_row = None;
                    break;
                }
            }
        }
        if let Some(row) = found_row {
            board.get_tile_mut(column, row).set_value(value);
            return true;
        }
    }
    false
}

/// Resolve tiles that have a note which is the only remaining note of that
/// value in this square.
/// Return after the first updated tile.
fn resolve_only_choice_in_square(board: &mut Board, square: usize) -> bool {
    for value in ALL_NUMBERS {
        // Count the number of tiles with this value in the notes
        let mut found_tile = None;
        for (column, row) in SquareIndexIter::new(square) {
            let tile = board.get_tile(column, row);
            let notes = tile.get_notes();
            if notes.contains(&value) {
                if found_tile.is_none() {
                    found_tile = Some((column, row));
                } else {
                    found_tile = None;
                    break;
                }
            }
        }
        if let Some((column, row)) = found_tile {
            board.get_tile_mut(column, row).set_value(value);
            return true;
        }
    }
    false
}

/// Resolve any tiles that are the only remaining option for a given value in
/// that tile's row, column, or square.
pub fn resolve_only_choice_remaining(board: &mut Board) -> bool {
    for row in 0..ROW_SIZE {
        if resolve_only_choice_in_row(board, row) {
            return true;
        }
    }
    for column in 0..ROW_SIZE {
        if resolve_only_choice_in_column(board, column) {
            return true;
        }
    }
    for square in 0..ROW_SIZE {
        if resolve_only_choice_in_square(board, square) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::Number;

    #[test]
    fn test_resolve_only_choice_in_row() {
        let mut board = Board::new();
        board.get_tile_mut(0, 0).set_note(Number::Seven);
        board.get_tile_mut(1, 0).set_note(Number::Seven);
        board.get_tile_mut(1, 0).set_note(Number::Eight);
        board.get_tile_mut(2, 0).set_note(Number::Seven);
        board.get_tile_mut(2, 0).set_note(Number::Eight);
        board.get_tile_mut(2, 0).set_note(Number::Nine);

        resolve_only_choice_in_row(&mut board, 0);
        assert_eq!(board.get_tile(0, 0).get_value(), None);
        assert_eq!(board.get_tile(1, 0).get_value(), None);
        assert_eq!(board.get_tile(2, 0).get_value(), Some(Number::Nine));
    }

    #[test]
    fn test_resolve_only_choice_in_column() {
        const COLUMN: usize = 3;
        let mut board = Board::new();
        board.get_tile_mut(COLUMN, 2).set_note(Number::Four);
        board.get_tile_mut(COLUMN, 3).set_note(Number::Four);
        board.get_tile_mut(COLUMN, 3).set_note(Number::Five);
        board.get_tile_mut(COLUMN, 3).set_note(Number::Six);
        board.get_tile_mut(COLUMN, 4).set_note(Number::Four);
        board.get_tile_mut(COLUMN, 4).set_note(Number::Five);

        resolve_only_choice_in_column(&mut board, COLUMN);
        assert_eq!(board.get_tile(COLUMN, 2).get_value(), None);
        assert_eq!(board.get_tile(COLUMN, 3).get_value(), Some(Number::Six));
        assert_eq!(board.get_tile(COLUMN, 4).get_value(), None);
    }

    #[test]
    fn test_resolve_only_choice_in_square() {
        const SQUARE: usize = 6;
        let mut board = Board::new();

        for (column, row) in SquareIndexIter::new(SQUARE) {
            board.get_tile_mut(column, row).set_note(Number::One);
            board.get_tile_mut(column, row).set_note(Number::Two);
        }
        board.get_tile_mut(1, 8).set_note(Number::Three);

        resolve_only_choice_in_square(&mut board, SQUARE);
        assert_eq!(board.get_tile(0, 6).get_value(), None);
        assert_eq!(board.get_tile(0, 7).get_value(), None);
        assert_eq!(board.get_tile(0, 8).get_value(), None);
        assert_eq!(board.get_tile(1, 6).get_value(), None);
        assert_eq!(board.get_tile(1, 7).get_value(), None);
        assert_eq!(board.get_tile(1, 8).get_value(), Some(Number::Three));
        assert_eq!(board.get_tile(2, 6).get_value(), None);
        assert_eq!(board.get_tile(2, 7).get_value(), None);
        assert_eq!(board.get_tile(2, 8).get_value(), None);
    }
}
