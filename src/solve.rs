//! Main entry point to solver

use crate::board::{Board, ROW_SIZE, SquareIndexIter, to_square_index};
use crate::debug::{print_board, print_notes};
use crate::number::Number;

fn clear_note_for_row(board: &mut Board, row: usize, value: Number) {
    for column in 0..ROW_SIZE {
        board.get_tile_mut(column, row).clear_note(value);
    }
}

fn clear_note_for_column(board: &mut Board, column: usize, value: Number) {
    for row in 0..ROW_SIZE {
        board.get_tile_mut(column, row).clear_note(value);
    }
}

fn clear_note_for_square(board: &mut Board, square: usize, value: Number) {
    let iter = SquareIndexIter::new(square);
    for (column, row) in iter {
        board.get_tile_mut(column, row).clear_note(value);
    }
}

fn simplify_notes(board: &mut Board) {
    for row in 0..ROW_SIZE {
        for col in 0..ROW_SIZE {
            if let Some(value) = board.rows[row].tiles[col].value {
                clear_note_for_row(board, row, value);
                clear_note_for_column(board, col, value);
                clear_note_for_square(board, to_square_index(col, row), value);
            }
        }
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

    while !board.is_solved() {
        print_board(board);
        print_notes(board);

        // TODO... (everything)

        simplify_notes(board);

        // Actually TODO:
        // 1. Debug function to print board with all notes
        // 2. Function to clear notes based on known values
        // 3. Function to set values based on only one note left.

        if prev_board == *board {
            return Err(eyre::eyre!("unsolvable!"));
        }
        prev_board = board.clone();
    }

    Ok(())
}
