//! Helper functions for solver: Check for obvious doubles

use crate::board::{Board, ROW_SIZE, SquareIndexIter};

/// Check for obvious doubles in the given row.
fn check_obvious_doubles_in_row(board: &mut Board, row: usize) -> bool {
    for column in 0..ROW_SIZE {
        let tile = board.get_tile(column, row);
        let notes = tile.get_notes();
        if notes.len() == 2 {
            // Check if other tiles have the same notes.
            // We only need to check tiles after this one, because tiles before
            // this one would have already matched.
            for column2 in (column + 1)..ROW_SIZE {
                let tile2 = board.get_tile(column2, row);
                let notes2 = tile2.get_notes();
                if notes == notes2 {
                    // Double found!
                    // Clear these notes from every other tile in this row.
                    let mut changes_found = false;
                    for column3 in 0..ROW_SIZE {
                        if column3 != column && column3 != column2 {
                            let tile3 = board.get_tile_mut(column3, row);
                            let notes_before = tile3.get_notes();
                            tile3.clear_note(notes[0]);
                            tile3.clear_note(notes[1]);
                            changes_found |= tile3.get_notes() != notes_before;
                        }
                    }
                    if changes_found {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Check for obvious doubles in the given column.
fn check_obvious_doubles_in_column(board: &mut Board, column: usize) -> bool {
    for row in 0..ROW_SIZE {
        let tile = board.get_tile(column, row);
        let notes = tile.get_notes();
        if notes.len() == 2 {
            // Check if other tiles have the same notes.
            // We only need to check tiles after this one, because tiles before
            // this one would have already matched.
            for row2 in (row + 1)..ROW_SIZE {
                let tile2 = board.get_tile(column, row2);
                let notes2 = tile2.get_notes();
                if notes == notes2 {
                    // Double found!
                    // Clear these notes from every other tile in this column.
                    let mut changes_found = false;
                    for row3 in 0..ROW_SIZE {
                        if row3 != row && row3 != row2 {
                            let tile3 = board.get_tile_mut(column, row3);
                            let notes_before = tile3.get_notes();
                            tile3.clear_note(notes[0]);
                            tile3.clear_note(notes[1]);
                            changes_found |= tile3.get_notes() != notes_before;
                        }
                    }
                    if changes_found {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Check for obvious doubles in the given square.
fn check_obvious_doubles_in_square(board: &mut Board, square: usize) -> bool {
    let mut iter = SquareIndexIter::new(square);
    while let Some((column, row)) = iter.next() {
        let tile = board.get_tile(column, row);
        let notes = tile.get_notes();
        if notes.len() == 2 {
            // Check if other tiles have the same notes.
            // We only need to check tiles after this one, because tiles before
            // this one would have already matched.
            let iter2 = iter.clone().skip(0);
            for (column2, row2) in iter2 {
                let tile2 = board.get_tile(column2, row2);
                let notes2 = tile2.get_notes();
                if notes == notes2 {
                    // Double found!
                    // Clear these notes from every other tile in this square.
                    let mut changes_found = false;
                    for (column3, row3) in SquareIndexIter::new(square) {
                        if !((column3 == column) && (row3 == row))
                            && !((column3 == column2) && (row3 == row2))
                        {
                            let tile3 = board.get_tile_mut(column3, row3);
                            let notes_before = tile3.get_notes();
                            tile3.clear_note(notes[0]);
                            tile3.clear_note(notes[1]);
                            changes_found |= tile3.get_notes() != notes_before;
                        }
                    }
                    if changes_found {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Check for obvious doubles (when a row, column, or square has two tiles with
/// the exact same two notes) and clear notes if doubles are found.
pub fn check_obvious_doubles(board: &mut Board) -> bool {
    for row in 0..ROW_SIZE {
        if check_obvious_doubles_in_row(board, row) {
            return true;
        }
    }
    for column in 0..ROW_SIZE {
        if check_obvious_doubles_in_column(board, column) {
            return true;
        }
    }
    for square in 0..ROW_SIZE {
        if check_obvious_doubles_in_square(board, square) {
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
    fn test_check_obvious_doubles_in_row() {
        let mut board = Board::new();
        board.get_tile_mut(0, 1).set_note(Number::Five);
        board.get_tile_mut(0, 1).set_note(Number::Seven);
        board.get_tile_mut(2, 1).set_note(Number::Five);
        board.get_tile_mut(2, 1).set_note(Number::Seven);
        board.get_tile_mut(7, 1).set_note(Number::Five);
        board.get_tile_mut(7, 1).set_note(Number::Seven);
        board.get_tile_mut(7, 1).set_note(Number::Eight);
        board.get_tile_mut(8, 1).set_note(Number::One);
        board.get_tile_mut(8, 1).set_note(Number::Two);
        board.get_tile_mut(8, 1).set_note(Number::Five);
        board.get_tile_mut(8, 1).set_note(Number::Seven);

        assert!(!check_obvious_doubles_in_row(&mut board, 0));
        assert!(check_obvious_doubles_in_row(&mut board, 1));

        assert_eq!(
            board.get_tile(0, 1).get_notes(),
            vec![Number::Five, Number::Seven]
        );
        assert_eq!(
            board.get_tile(2, 1).get_notes(),
            vec![Number::Five, Number::Seven]
        );
        assert_eq!(board.get_tile(7, 1).get_notes(), vec![Number::Eight]);
        assert_eq!(
            board.get_tile(8, 1).get_notes(),
            vec![Number::One, Number::Two]
        );
    }

    #[test]
    fn test_check_obvious_doubles_in_column() {
        let mut board = Board::new();
        board.get_tile_mut(5, 1).set_notes_u8(&[3, 6, 9]);
        board.get_tile_mut(5, 3).set_notes_u8(&[6, 9]);
        board.get_tile_mut(5, 5).set_notes_u8(&[2, 3, 6, 9]);
        board.get_tile_mut(5, 7).set_notes_u8(&[6, 9]);

        assert!(!check_obvious_doubles_in_column(&mut board, 0));
        assert!(!check_obvious_doubles_in_column(&mut board, 1));
        assert!(!check_obvious_doubles_in_column(&mut board, 2));
        assert!(!check_obvious_doubles_in_column(&mut board, 3));
        assert!(!check_obvious_doubles_in_column(&mut board, 4));
        assert!(check_obvious_doubles_in_column(&mut board, 5));
        assert!(!check_obvious_doubles_in_column(&mut board, 6));
        assert!(!check_obvious_doubles_in_column(&mut board, 7));
        assert!(!check_obvious_doubles_in_column(&mut board, 8));

        assert_eq!(board.get_tile(5, 1).get_notes(), vec![Number::Three]);
        assert_eq!(
            board.get_tile(5, 3).get_notes(),
            vec![Number::Six, Number::Nine]
        );
        assert_eq!(
            board.get_tile(5, 5).get_notes(),
            vec![Number::Two, Number::Three]
        );
        assert_eq!(
            board.get_tile(5, 7).get_notes(),
            vec![Number::Six, Number::Nine]
        );
    }

    #[test]
    fn test_check_obvious_doubles_in_square() {
        let mut board = Board::new();
        board.get_tile_mut(6, 6).set_notes_u8(&[1, 5, 8]);
        board.get_tile_mut(7, 6).set_notes_u8(&[1, 8]);
        board.get_tile_mut(8, 6).set_notes_u8(&[1, 7, 8]);
        board.get_tile_mut(6, 7).set_notes_u8(&[1, 2, 8]);
        board.get_tile_mut(7, 7).set_notes_u8(&[1, 8]);
        board.get_tile_mut(8, 7).set_notes_u8(&[1, 3, 6, 7, 8]);
        board.get_tile_mut(6, 8).set_notes_u8(&[1, 3, 8]);
        board.get_tile_mut(7, 8).set_notes_u8(&[1, 4, 5, 8]);
        board.get_tile_mut(8, 8).set_notes_u8(&[1, 4, 5, 6, 8]);

        assert!(!check_obvious_doubles_in_square(&mut board, 0));
        assert!(!check_obvious_doubles_in_square(&mut board, 1));
        assert!(!check_obvious_doubles_in_square(&mut board, 2));
        assert!(!check_obvious_doubles_in_square(&mut board, 3));
        assert!(!check_obvious_doubles_in_square(&mut board, 4));
        assert!(!check_obvious_doubles_in_square(&mut board, 5));
        assert!(!check_obvious_doubles_in_square(&mut board, 6));
        assert!(!check_obvious_doubles_in_square(&mut board, 7));
        assert!(check_obvious_doubles_in_square(&mut board, 8));

        assert_eq!(board.get_tile(6, 6).get_notes(), vec![Number::Five]);
        assert_eq!(
            board.get_tile(7, 6).get_notes(),
            vec![Number::One, Number::Eight]
        );
        assert_eq!(board.get_tile(8, 6).get_notes(), vec![Number::Seven]);
        assert_eq!(board.get_tile(6, 7).get_notes(), vec![Number::Two]);
        assert_eq!(
            board.get_tile(7, 7).get_notes(),
            vec![Number::One, Number::Eight]
        );
        assert_eq!(
            board.get_tile(8, 7).get_notes(),
            vec![Number::Three, Number::Six, Number::Seven]
        );
        assert_eq!(board.get_tile(6, 8).get_notes(), vec![Number::Three]);
        assert_eq!(
            board.get_tile(7, 8).get_notes(),
            vec![Number::Four, Number::Five]
        );
        assert_eq!(
            board.get_tile(8, 8).get_notes(),
            vec![Number::Four, Number::Five, Number::Six]
        );
    }

    #[test]
    fn test_check_obvious_doubles_in_square2() {
        let mut board = Board::new();

        // Row 0
        board.get_tile_mut(0, 0).set_value_u8(2);
        board.get_tile_mut(1, 0).set_value_u8(6);
        board.get_tile_mut(2, 0).set_notes_u8(&[3, 8, 9]);
        board.get_tile_mut(3, 0).set_value_u8(5);
        board.get_tile_mut(4, 0).set_value_u8(1);
        board.get_tile_mut(5, 0).set_value_u8(4);
        board.get_tile_mut(6, 0).set_notes_u8(&[3, 8, 9]);
        board.get_tile_mut(7, 0).set_value_u8(7);
        board.get_tile_mut(8, 0).set_notes_u8(&[3, 8, 9]);

        // Row 1
        board.get_tile_mut(0, 1).set_notes_u8(&[5, 9]);
        board.get_tile_mut(1, 1).set_notes_u8(&[1, 5, 8]);
        board.get_tile_mut(2, 1).set_notes_u8(&[1, 5, 8, 9]);
        board.get_tile_mut(3, 1).set_value_u8(3);
        board.get_tile_mut(4, 1).set_value_u8(6);
        board.get_tile_mut(5, 1).set_value_u8(7);
        board.get_tile_mut(6, 1).set_notes_u8(&[1, 8, 9]);
        board.get_tile_mut(7, 1).set_value_u8(2);
        board.get_tile_mut(8, 1).set_value_u8(4);

        // Row 2
        board.get_tile_mut(0, 2).set_value_u8(4);
        board.get_tile_mut(1, 2).set_notes_u8(&[1, 7]);
        board.get_tile_mut(2, 2).set_notes_u8(&[1, 3, 7]);
        board.get_tile_mut(3, 2).set_notes_u8(&[8, 9]);
        board.get_tile_mut(4, 2).set_notes_u8(&[2, 8, 9]);
        board.get_tile_mut(5, 2).set_notes_u8(&[2, 8, 9]);
        board.get_tile_mut(6, 2).set_notes_u8(&[1, 3]);
        board.get_tile_mut(7, 2).set_value_u8(6);
        board.get_tile_mut(8, 2).set_value_u8(5);

        // Row 3
        board.get_tile_mut(0, 3).set_notes_u8(&[3, 5, 6, 7]);
        board.get_tile_mut(1, 3).set_value_u8(9);
        board.get_tile_mut(2, 3).set_notes_u8(&[4, 5, 6, 7]);
        board.get_tile_mut(3, 3).set_notes_u8(&[7, 8]);
        board.get_tile_mut(4, 3).set_notes_u8(&[5, 7, 8]);
        board.get_tile_mut(5, 3).set_value_u8(1);
        board.get_tile_mut(6, 3).set_value_u8(2);
        board.get_tile_mut(7, 3).set_notes_u8(&[3, 4, 8]);
        board.get_tile_mut(8, 3).set_notes_u8(&[3, 7, 8]);

        // Row 4
        board.get_tile_mut(0, 4).set_value_u8(8);
        board.get_tile_mut(1, 4).set_notes_u8(&[1, 4, 7]);
        board.get_tile_mut(2, 4).set_value_u8(2);
        board.get_tile_mut(3, 4).set_value_u8(6);
        board.get_tile_mut(4, 4).set_notes_u8(&[7, 9]);
        board.get_tile_mut(5, 4).set_value_u8(3);
        board.get_tile_mut(6, 4).set_value_u8(5);
        board.get_tile_mut(7, 4).set_notes_u8(&[1, 4, 9]);
        board.get_tile_mut(8, 4).set_notes_u8(&[1, 7, 9]);

        // Row 5
        board.get_tile_mut(0, 5).set_notes_u8(&[3, 5, 7]);
        board.get_tile_mut(1, 5).set_notes_u8(&[1, 5, 7]);
        board.get_tile_mut(2, 5).set_notes_u8(&[1, 5, 7]);
        board.get_tile_mut(3, 5).set_value_u8(4);
        board.get_tile_mut(4, 5).set_notes_u8(&[2, 5, 7, 8, 9]);
        board.get_tile_mut(5, 5).set_notes_u8(&[2, 5, 8, 9]);
        board.get_tile_mut(6, 5).set_value_u8(6);
        board.get_tile_mut(7, 5).set_notes_u8(&[1, 3, 8, 9]);
        board.get_tile_mut(8, 5).set_notes_u8(&[1, 3, 7, 8, 9]);

        // Row 6
        board.get_tile_mut(0, 6).set_value_u8(1);
        board.get_tile_mut(1, 6).set_value_u8(3);
        board.get_tile_mut(2, 6).set_notes_u8(&[4, 5, 7]);
        board.get_tile_mut(3, 6).set_value_u8(2);
        board.get_tile_mut(4, 6).set_notes_u8(&[5, 7]);
        board.get_tile_mut(5, 6).set_value_u8(6);
        board.get_tile_mut(6, 6).set_notes_u8(&[4, 7]);
        board.get_tile_mut(7, 6).set_notes_u8(&[8, 9]);
        board.get_tile_mut(8, 6).set_notes_u8(&[8, 9]);

        // Row 7
        board.get_tile_mut(0, 7).set_notes_u8(&[5, 6, 7, 9]);
        board.get_tile_mut(1, 7).set_notes_u8(&[2, 5, 7, 8]);
        board.get_tile_mut(2, 7).set_notes_u8(&[5, 6, 7, 8, 9]);
        board.get_tile_mut(3, 7).set_notes_u8(&[1, 7, 8, 9]);
        board.get_tile_mut(4, 7).set_value_u8(4);
        board.get_tile_mut(5, 7).set_notes_u8(&[5, 8, 9]);
        board.get_tile_mut(6, 7).set_notes_u8(&[3, 7, 8, 9]);
        board.get_tile_mut(7, 7).set_notes_u8(&[1, 3, 8, 9]);
        board.get_tile_mut(8, 7).set_notes_u8(&[1, 2, 3, 6, 8, 9]);

        // Row 8
        board.get_tile_mut(0, 8).set_notes_u8(&[6, 7, 9]);
        board.get_tile_mut(1, 8).set_notes_u8(&[2, 4, 7, 8]);
        board.get_tile_mut(2, 8).set_notes_u8(&[4, 6, 7, 8, 9]);
        board.get_tile_mut(3, 8).set_notes_u8(&[1, 7, 8, 9]);
        board.get_tile_mut(4, 8).set_value_u8(3);
        board.get_tile_mut(5, 8).set_notes_u8(&[8, 9]);
        board.get_tile_mut(6, 8).set_notes_u8(&[4, 7, 8, 9]);
        board.get_tile_mut(7, 8).set_value_u8(5);
        board.get_tile_mut(8, 8).set_notes_u8(&[1, 2, 6, 8, 9]);

        // Check for double
        assert!(check_obvious_doubles_in_square(&mut board, 8));
    }
}
