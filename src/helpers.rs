//! Helper functions for modifying sudoku board.

use crate::board::{Board, ROW_SIZE, SquareIndexIter};
use crate::number::Number;

/// Clear the given value from the notes of every tile in this row.
///
/// * `board` - Board to update
/// * `row` - Row to clear
/// * `value` - Value of note to clear
///
pub fn clear_note_for_row(board: &mut Board, row: usize, value: Number) {
    for column in 0..ROW_SIZE {
        board.get_tile_mut(column, row).clear_note(value);
    }
}

/// Clear the given value from the notes of every tile in this Column.
///
/// * `board` - Board to update
/// * `column` - Column to clear
/// * `value` - Value of note to clear
///
pub fn clear_note_for_column(board: &mut Board, column: usize, value: Number) {
    for row in 0..ROW_SIZE {
        board.get_tile_mut(column, row).clear_note(value);
    }
}

/// Clear the given value from the notes of every tile in this square.
///
/// * `board` - Board to update
/// * `square` - Square to clear
/// * `value` - Value of note to clear
///
pub fn clear_note_for_square(board: &mut Board, square: usize, value: Number) {
    let iter = SquareIndexIter::new(square);
    for (column, row) in iter {
        board.get_tile_mut(column, row).clear_note(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_note_for_row() {
        let mut board = Board::new();

        board.get_tile_mut(3, 3).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(3, 4).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(3, 5).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(4, 3).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(4, 4).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(4, 5).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(5, 3).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(5, 4).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(5, 5).set_notes_u8(&[5, 7, 9]);

        clear_note_for_row(&mut board, 4, Number::Five);

        assert_eq!(board.get_tile(3, 3).get_notes_u8(), &[5, 7, 9]);
        assert_eq!(board.get_tile(3, 4).get_notes_u8(), &[7, 9]);
        assert_eq!(board.get_tile(3, 5).get_notes_u8(), &[5, 7, 9]);
        assert_eq!(board.get_tile(4, 3).get_notes_u8(), &[5, 7, 9]);
        assert_eq!(board.get_tile(4, 4).get_notes_u8(), &[7, 9]);
        assert_eq!(board.get_tile(4, 5).get_notes_u8(), &[5, 7, 9]);
        assert_eq!(board.get_tile(5, 3).get_notes_u8(), &[5, 7, 9]);
        assert_eq!(board.get_tile(5, 4).get_notes_u8(), &[7, 9]);
        assert_eq!(board.get_tile(5, 5).get_notes_u8(), &[5, 7, 9]);
    }

    #[test]
    fn test_clear_note_for_column() {
        let mut board = Board::new();

        board.get_tile_mut(3, 3).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(3, 4).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(3, 5).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(4, 3).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(4, 4).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(4, 5).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(5, 3).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(5, 4).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(5, 5).set_notes_u8(&[5, 7, 9]);

        clear_note_for_column(&mut board, 4, Number::Seven);

        assert_eq!(board.get_tile(3, 3).get_notes_u8(), &[5, 7, 9]);
        assert_eq!(board.get_tile(3, 4).get_notes_u8(), &[5, 7, 9]);
        assert_eq!(board.get_tile(3, 5).get_notes_u8(), &[5, 7, 9]);
        assert_eq!(board.get_tile(4, 3).get_notes_u8(), &[5, 9]);
        assert_eq!(board.get_tile(4, 4).get_notes_u8(), &[5, 9]);
        assert_eq!(board.get_tile(4, 5).get_notes_u8(), &[5, 9]);
        assert_eq!(board.get_tile(5, 3).get_notes_u8(), &[5, 7, 9]);
        assert_eq!(board.get_tile(5, 4).get_notes_u8(), &[5, 7, 9]);
        assert_eq!(board.get_tile(5, 5).get_notes_u8(), &[5, 7, 9]);
    }

    #[test]
    fn test_clear_note_for_square() {
        let mut board = Board::new();

        board.get_tile_mut(3, 3).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(3, 4).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(3, 5).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(4, 3).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(4, 4).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(4, 5).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(5, 3).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(5, 4).set_notes_u8(&[5, 7, 9]);
        board.get_tile_mut(5, 5).set_notes_u8(&[5, 7, 9]);

        clear_note_for_square(&mut board, 4, Number::Nine);

        assert_eq!(board.get_tile(3, 3).get_notes_u8(), &[5, 7]);
        assert_eq!(board.get_tile(3, 4).get_notes_u8(), &[5, 7]);
        assert_eq!(board.get_tile(3, 5).get_notes_u8(), &[5, 7]);
        assert_eq!(board.get_tile(4, 3).get_notes_u8(), &[5, 7]);
        assert_eq!(board.get_tile(4, 4).get_notes_u8(), &[5, 7]);
        assert_eq!(board.get_tile(4, 5).get_notes_u8(), &[5, 7]);
        assert_eq!(board.get_tile(5, 3).get_notes_u8(), &[5, 7]);
        assert_eq!(board.get_tile(5, 4).get_notes_u8(), &[5, 7]);
        assert_eq!(board.get_tile(5, 5).get_notes_u8(), &[5, 7]);
    }
}
