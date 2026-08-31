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
                    for column3 in 0..ROW_SIZE {
                        if column3 != column && column3 != column2 {
                            let tile3 = board.get_tile_mut(column3, row);
                            tile3.clear_note(notes[0]);
                            tile3.clear_note(notes[1]);
                            return true;
                        }
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
                    for row3 in 0..ROW_SIZE {
                        if row3 != row && row3 != row2 {
                            let tile3 = board.get_tile_mut(column, row3);
                            tile3.clear_note(notes[0]);
                            tile3.clear_note(notes[1]);
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

/// Check for obvious doubles in the given square.
fn check_obvious_doubles_in_square(board: &mut Board, square: usize) -> bool {
    let iter = SquareIndexIter::new(square);
    for (column, row) in iter {
        let tile = board.get_tile(column, row);
        let notes = tile.get_notes();
        if notes.len() == 2 {
            // Check if other tiles have the same notes.
            // We only need to check tiles after this one, because tiles before
            // this one would have already matched.
            let iter2 = iter.clone().skip(1);
            for (column2, row2) in iter2 {
                let tile2 = board.get_tile(column2, row2);
                let notes2 = tile2.get_notes();
                if notes == notes2 {
                    // Double found!
                    // Clear these notes from every other tile in this square.
                    let iter3 = SquareIndexIter::new(square);
                    for (column3, row3) in iter3 {
                        if ((column3, row3) != (column, row))
                            && ((column3, row3) != (column2, row2))
                        {
                            let tile3 = board.get_tile_mut(column3, row3);
                            tile3.clear_note(notes[0]);
                            tile3.clear_note(notes[1]);
                            return true;
                        }
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
