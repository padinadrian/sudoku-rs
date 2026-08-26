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
            if let Some(value) = board.rows[row].tiles[col].get_value() {
                clear_note_for_row(board, row, value);
                clear_note_for_column(board, col, value);
                clear_note_for_square(board, to_square_index(col, row), value);
            }
        }
    }
}

/// Resolve any tiles that only have a single note and mark them with that value.
fn resolve_single_notes(board: &mut Board) {
    for row in 0..ROW_SIZE {
        for col in 0..ROW_SIZE {
            let tile = board.get_tile_mut(col, row);
            if tile.get_value().is_none() {
                let notes = tile.get_notes();
                if notes.len() == 1 {
                    let value = notes[0];
                    tile.set_value(value);
                    tile.clear_note(value);
                }
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

    let mut counter = 0;

    while !board.is_solved() {
        counter += 1;

        print_board(board);
        print_notes(board);

        // TODO... (everything)

        simplify_notes(board);
        resolve_single_notes(board);

        // Actually TODO:
        // 1. Debug function to print board with all notes
        // 2. Function to clear notes based on known values
        // 3. Function to set values based on only one note left.

        if prev_board == *board {
            println!("Failed to solve after {} iterations.", counter);
            return Err(eyre::eyre!("unsolvable!"));
        }
        prev_board = board.clone();
    }

    print_board(board);
    println!("Solved after {} iterations.", counter);

    Ok(())
}
