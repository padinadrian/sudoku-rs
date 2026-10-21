//! Helper functions for solver - Check for sweeping lines in squares.

//! A sweeping line is a situation where a given square has notes that only
//! appear in a single line (vertical or horizon). Since that guarantees that
//! one of the values for that square must be taken in that row or column, no
//! other tiles in that row or column can be that value.

use crate::Board;
use crate::board::{ROW_SIZE, SquareIndexIter};
use crate::number::Number;

/// Get the index for all tiles in this square that have a
/// note for the given value.
///
/// * `board` - Board to check
/// * `square` - Index of square to check (0 through 8 inclusive)
/// * `value` - Value of note to look for
///
fn matching_tiles(board: &Board, square: usize, value: Number) -> Vec<(usize, usize)> {
    SquareIndexIter::new(square)
        .filter(|&(column, row)| board.get_tile(column, row).has_note(value))
        .collect()
}

/// Check for "sweeping lines" in the board at the given square.
///
/// A sweeping line is where the only tiles with a given
/// note value in a square all fall in the same row or column.
/// (Since this excludes single values, there will be 2 or 3 tiles in a line.)
///
/// In this scenario, since the number for that square must be in one of those
/// tiles, the other tiles in that row or column cannot contain that number,
/// so those notes can be cleared of that value.
///
/// * `board` - Board to check and update
/// * `square` - Index of the square to check (0 through 8 inclusive)
///
fn check_sweeping_lines_in_square(board: &mut Board, square: usize) -> bool {
    for value in crate::number::ALL_NUMBERS {
        // Get all tiles with a note for this
        let coords = matching_tiles(board, square, value);
        if coords.len() > 1 {
            let (column, row) = coords[0];
            let mut change_made = false;
            let all_same_row = coords[1..].iter().all(|&(_, r)| r == row);
            if all_same_row {
                // Possible sweeping row identified
                let skip_columns: Vec<_> = coords.iter().map(|&(c, _)| c).collect();
                for column2 in 0..ROW_SIZE {
                    if !skip_columns.contains(&column2) {
                        let tile = board.get_tile_mut(column2, row);
                        if tile.has_note(value) {
                            tile.clear_note(value);
                            change_made |= true;
                        }
                    }
                }
                if change_made {
                    return true;
                }
            }
            let all_same_col = coords[1..].iter().all(|&(c, _)| c == column);
            if all_same_col {
                // Possible sweeping column identified
                let skip_rows: Vec<_> = coords.iter().map(|&(_, r)| r).collect();
                for row2 in 0..ROW_SIZE {
                    if !skip_rows.contains(&row2) {
                        let tile = board.get_tile_mut(column, row2);
                        if tile.has_note(value) {
                            tile.clear_note(value);
                            change_made |= true;
                        }
                    }
                }
                if change_made {
                    return true;
                }
            }
        }
    }
    false
}

pub fn check_sweeping_lines(board: &mut Board) -> bool {
    for square in 0..ROW_SIZE {
        if check_sweeping_lines_in_square(board, square) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    /* ===== Tests for matching_tiles() =====  */

    #[test]
    fn test_matching_tiles1() {
        let mut board = Board::new();
        board.fill_all_notes();
        let matching = matching_tiles(&board, 0, Number::Two);
        assert_eq!(matching.len(), 9);
    }

    #[test]
    fn test_matching_tiles2() {
        let mut board = Board::new();
        board.get_tile_mut(2, 3).set_note_u8(5);
        board.get_tile_mut(3, 3).set_note_u8(5);
        board.get_tile_mut(4, 4).set_note_u8(5);
        board.get_tile_mut(5, 5).set_note_u8(5);
        board.get_tile_mut(5, 6).set_note_u8(5);
        board.get_tile_mut(5, 7).set_note_u8(5);
        let matching_square3 = matching_tiles(&board, 3, Number::Five);
        assert_eq!(matching_square3.len(), 1);
        let matching_square4 = matching_tiles(&board, 4, Number::Five);
        assert_eq!(matching_square4.len(), 3);
        let matching_square7 = matching_tiles(&board, 7, Number::Five);
        assert_eq!(matching_square7.len(), 2);
        let matching_square8 = matching_tiles(&board, 8, Number::Five);
        assert_eq!(matching_square8.len(), 0);
    }

    /* ===== Tests for check_sweeping_lines_in_square() =====  */

    #[test]
    fn check_sweeping_lines_in_square1() {
        let mut board = Board::new();
        board.get_tile_mut(0, 0).set_notes_u8(&[1, 2, 3]);
        board.get_tile_mut(1, 0).set_notes_u8(&[1, 2]);
        board.get_tile_mut(2, 0).set_notes_u8(&[1, 2, 3]);
        board.get_tile_mut(3, 0).set_notes_u8(&[1, 3]);
        board.get_tile_mut(0, 1).set_notes_u8(&[1, 2]);

        // Returns true if a change was made
        assert!(check_sweeping_lines_in_square(&mut board, 0));

        assert_eq!(board.get_tile(0, 0).get_notes_u8(), vec![1, 2, 3]);
        assert_eq!(board.get_tile(1, 0).get_notes_u8(), vec![1, 2]);
        assert_eq!(board.get_tile(2, 0).get_notes_u8(), vec![1, 2, 3]);
        assert_eq!(board.get_tile(3, 0).get_notes_u8(), vec![1]);
        assert_eq!(board.get_tile(0, 1).get_notes_u8(), vec![1, 2]);
    }

    #[test]
    fn check_sweeping_lines_in_square2() {
        let mut board = Board::new();
        board.get_tile_mut(7, 2).set_notes_u8(&[6, 7, 8]);
        board.get_tile_mut(7, 3).set_notes_u8(&[6, 7, 8]);
        board.get_tile_mut(7, 4).set_notes_u8(&[6, 7, 8]);
        board.get_tile_mut(7, 5).set_notes_u8(&[7, 8]);
        board.get_tile_mut(7, 6).set_notes_u8(&[6, 7]);
        board.get_tile_mut(6, 3).set_notes_u8(&[6, 8]);

        // Returns true if a change was made
        assert!(check_sweeping_lines_in_square(&mut board, 5));

        assert_eq!(board.get_tile(7, 2).get_notes_u8(), vec![6, 8]);
        assert_eq!(board.get_tile(7, 3).get_notes_u8(), vec![6, 7, 8]);
        assert_eq!(board.get_tile(7, 4).get_notes_u8(), vec![6, 7, 8]);
        assert_eq!(board.get_tile(7, 5).get_notes_u8(), vec![7, 8]);
        assert_eq!(board.get_tile(7, 6).get_notes_u8(), vec![6]);
        assert_eq!(board.get_tile(6, 3).get_notes_u8(), vec![6, 8]);
    }
}
