//! Helpers for solver - Check for only lines

use crate::board::{Board, ROW_SIZE, RowIter};
use crate::number::{ALL_NUMBERS, Number};

/// Get the index for all tiles in this row that have a
/// note for the given value.
///
/// * `board` - Board to check
/// * `row` - Index of row to check (0 through 8 inclusive)
/// * `value` - Value of note to look for
///
fn matching_tiles_in_row(board: &Board, row: usize, value: Number) -> Vec<(usize, usize)> {
    // board
    //     .get_row(row)
    //     .filter(|tile| tile.has_note(value))
    //     .collect()
    vec![]
}

/// Check for only lines in the given row.
pub fn check_only_lines_in_row(board: &mut Board, row: usize) -> bool {
    false
}

/// Check for only lines in the given column.
pub fn check_only_lines_in_column(board: &mut Board, column: usize) -> bool {
    // Check if all notes for the given value are in the same square
    for value in ALL_NUMBERS {}

    false
}

pub fn check_only_lines(board: &mut Board) -> bool {
    for row in 0..ROW_SIZE {
        if check_only_lines_in_row(board, row) {
            return true;
        }
    }
    false
}
