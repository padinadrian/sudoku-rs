//! Main entry point to solver

use crate::board::{Board, ROW_SIZE, SquareIndexIter, to_square_index};
use crate::debug::{print_board, print_notes};
use crate::number::{ALL_NUMBERS, Number};

/// Clear the given value from the notes of every tile in this row.
fn clear_note_for_row(board: &mut Board, row: usize, value: Number) {
    for column in 0..ROW_SIZE {
        board.get_tile_mut(column, row).clear_note(value);
    }
}

/// Clear the given value from the notes of every tile in this Column.
fn clear_note_for_column(board: &mut Board, column: usize, value: Number) {
    for row in 0..ROW_SIZE {
        board.get_tile_mut(column, row).clear_note(value);
    }
}

/// Clear the given value from the notes of every tile in this square.
fn clear_note_for_square(board: &mut Board, square: usize, value: Number) {
    let iter = SquareIndexIter::new(square);
    for (column, row) in iter {
        board.get_tile_mut(column, row).clear_note(value);
    }
}

/// Clear the notes for tiles that have another tile in the same row, column,
/// or square that already have that value set.
fn simplify_notes(board: &mut Board) {
    for row in 0..ROW_SIZE {
        for col in 0..ROW_SIZE {
            if let Some(value) = board.get_value(col, row) {
                clear_note_for_row(board, row, value);
                clear_note_for_column(board, col, value);
                clear_note_for_square(board, to_square_index(col, row), value);
            }
        }
    }
}

/// Resolve tiles that only have a single note and mark them with that value.
/// Stop after the first updated tile.
fn resolve_single_notes(board: &mut Board) -> bool {
    for row in 0..ROW_SIZE {
        for col in 0..ROW_SIZE {
            let tile = board.get_tile_mut(col, row);
            if tile.get_value().is_none() {
                let notes = tile.get_notes();
                if notes.len() == 1 {
                    let value = notes[0];
                    tile.set_value(value);
                    tile.clear_note(value);
                    return true;
                }
            }
        }
    }
    false
}

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
fn resolve_only_choice_remaining(board: &mut Board) -> bool {
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

/// Check for obvious doubles in the given row.
fn check_obvious_doubles_in_row(board: &mut Board, row: usize) {
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
                            return;
                        }
                    }
                }
            }
        }
    }
}

/// Check for obvious doubles in the given column.
fn check_obvious_doubles_in_column(board: &mut Board, column: usize) {
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
                            return;
                        }
                    }
                }
            }
        }
    }
}

/// Check for obvious doubles in the given square.
fn check_obvious_doubles_in_square(board: &mut Board, square: usize) {
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
                            return;
                        }
                    }
                }
            }
        }
    }
}

/// Check for obvious doubles (when a row, column, or square has two tiles with
/// the exact same two notes) and clear notes if doubles are found.
fn check_obvious_doubles(board: &mut Board) {
    for row in 0..ROW_SIZE {
        check_obvious_doubles_in_row(board, row);
    }
    for column in 0..ROW_SIZE {
        check_obvious_doubles_in_column(board, column);
    }
    for square in 0..ROW_SIZE {
        check_obvious_doubles_in_square(board, square);
    }
}

/// Solve the board
/// Returns Ok() on success or Err on failure.
pub fn solve(board: &mut Board) -> color_eyre::Result<()> {
    if board.is_solved() {
        return Ok(());
    }

    board.fill_all_notes();

    let mut prev_board = board.clone();

    let mut counter = 0;

    while !board.is_solved() {
        counter += 1;

        // TODO: Debugging only, remove later
        print_board(board);
        print_notes(board);

        // Always do this step
        simplify_notes(board);

        // Only do one of these at a time so we can keep track of the steps
        // more easily.
        if resolve_single_notes(board) {
            continue;
        }
        if resolve_only_choice_remaining(board) {
            continue;
        }
        // check_obvious_doubles(board);

        // Actually TODO:
        // 1. Debug function to print board with all notes
        // 2. Function to clear notes based on known values
        // 3. Function to set values based on only one note left.

        if prev_board == *board {
            print_board(board);
            println!("Failed to solve after {} iterations.", counter);
            return Err(eyre::eyre!("unsolvable!"));
        }
        prev_board = board.clone();
    }

    print_board(board);
    println!("Solved after {} iterations.", counter);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_single_notes() {
        let mut board = Board::new();
        board.get_tile_mut(0, 0).set_note(Number::Seven);
        board.get_tile_mut(0, 1).set_note(Number::Seven);
        board.get_tile_mut(0, 1).set_note(Number::Eight);
        board.get_tile_mut(0, 2).set_note(Number::Seven);
        board.get_tile_mut(0, 2).set_note(Number::Eight);
        board.get_tile_mut(0, 2).set_note(Number::Nine);

        resolve_single_notes(&mut board);
        assert_eq!(board.get_tile(0, 0).get_value(), Some(Number::Seven));
        assert_eq!(board.get_tile(0, 1).get_value(), None);
        assert_eq!(board.get_tile(0, 2).get_value(), None);
    }

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
        print_notes(&board);
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
