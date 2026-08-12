//! Main entry point to solver

use crate::board::Board;
use crate::debug::{print_board, print_notes};

/// Solve the board
/// Returns Ok() on success or Err on failure.
pub fn solve(board: &mut Board) -> color_eyre::Result<()> {
    if board.is_solved() {
        return Ok(());
    }

    board.fill_all_notes();

    while !board.is_solved() {
        print_board(board);
        print_notes(board);

        // TODO... (everything)

        // Actually TODO:
        // 1. Debug function to print board with all notes
        // 2. Function to clear notes based on known values
        // 3. Function to set values based on only one note left.

        return Err(eyre::eyre!("unsolvable!"));
    }

    Ok(())
}
