//! Main entry point to solver

mod obvious_doubles;
mod obvious_triples;
mod only_choice;
mod sweeping_lines;

use crate::board::{Board, ROW_SIZE, to_square_index};
use crate::debug::{print_board, print_notes};
use crate::helpers::{clear_note_for_column, clear_note_for_row, clear_note_for_square};
use crate::solve::sweeping_lines::check_sweeping_lines;
use obvious_doubles::check_obvious_doubles;
use obvious_triples::check_obvious_triples;
use only_choice::resolve_only_choice_remaining;

/// Solve the board
/// Returns Ok() on success or Err on failure.
///
pub fn solve(board: &mut Board) -> color_eyre::Result<()> {
    if board.is_solved() {
        return Ok(());
    }

    // Only do this once
    board.fill_all_notes();

    let mut counter = 0;
    let result = solve_loop(board, &mut counter);

    match result {
        Ok(_) => {
            print_board(board);
            println!("Solved after {} iterations.", counter);
        }
        Err(e) => {
            print_board(board);
            println!("Failed to solve after {} iterations.", counter);
            println!("Result: {}", e);
        }
    };

    Ok(())
}

/// Run the loop of trying to solve the puzzle.
///
/// * `board` - Board to solve
/// * `counter` - Running count of iterations
///
fn solve_loop(board: &mut Board, counter: &mut u32) -> color_eyre::Result<()> {
    let mut prev_board = board.clone();

    while !board.is_solved() {
        println!("Iteration: {}", counter);
        *counter += 1;

        let _changed = solve_iteration(board);

        if is_unsolvable(board) {
            return Err(eyre::eyre!("Unsolvable"));
        }

        if prev_board == *board {
            // No progress made from heuristic methods
            // Fall back to Bowman's Bingo - i.e. guess and check
            let result = bowmans_bingo(board, counter);

            if result.is_err() {
                print_board(board);
                print_notes(board);
                println!("Failed to solve after {} iterations.", counter);
                return result;
            }
        }
        prev_board = board.clone();
    }

    Ok(())
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
///
/// * `board` - Board to resolve
///
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

/// Check if the board has reached a dead-end unsolvable state.
///
/// A board is considered unsolvable if a tile has no value and no remaining
/// possible values, i.e. if the notes are empty
///
fn is_unsolvable(board: &Board) -> bool {
    for x in 0..ROW_SIZE {
        for y in 0..ROW_SIZE {
            let tile = board.get_tile(x, y);
            if tile.get_value().is_none() && tile.get_notes().is_empty() {
                return true;
            }
        }
    }
    false
}

/// Run one iteration of the solver to make a single change.
/// Return true if a change was made, otherwise false.
///
/// * `board` - Board to resolve
///
fn solve_iteration(board: &mut Board) -> bool {
    // Always do this step because it doesn't set any values, it only
    // changes the notes.
    simplify_notes(board);

    // Only do one of these at a time so we can keep track of the steps
    // more easily.
    if resolve_single_notes(board) {
        // println!("Resolved single note.");
        return true;
    }
    if resolve_only_choice_remaining(board) {
        // println!("Resolved only choice remaining.");
        return true;
    }
    if check_obvious_doubles(board) {
        // println!("Obvious double found.");
        return true;
    }
    if check_obvious_triples(board) {
        // println!("Obvious triple found.");
        return true;
    }
    if check_sweeping_lines(board) {
        // println!("Sweeping line found.");
        return true;
    }

    // Solver steps TODO:
    //  * Find hidden doubles
    //  * Find hidden triples
    //  * X-ray
    //  * Bowman's bingo

    return false;
}

/// Run the Bowman's Bingo algorithm - in other words, guess and check.
///
pub fn bowmans_bingo(board: &mut Board, counter: &mut u32) -> color_eyre::Result<()> {
    // Find an unsolved tile and pick one of the notes to guess
    let find_guess = || {
        for y in 0..ROW_SIZE {
            for x in 0..ROW_SIZE {
                let notes = board.get_tile(x, y).get_notes();
                if !notes.is_empty() {
                    return Ok((x, y, notes));
                }
            }
        }
        return Err(eyre::eyre!("No tiles left with notes"));
    };
    let (x, y, notes) = find_guess()?;
    for value in notes {
        let board_copy = board.clone();
        println!("Bowman's Bingo: trying ({}, {}) -> {}", x, y, value);
        board.get_tile_mut(x, y).set_value(value);
        let result = solve_loop(board, counter);
        if result.is_ok() {
            return Ok(());
        } else {
            *board = board_copy;
        }
    }
    Err(eyre::eyre!("Bowman's Bingo failed"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::Number;

    #[test]
    fn test_resolve_single_notes() {
        let mut board = Board::new();
        board.get_tile_mut(0, 0).set_note_u8(7);
        board.get_tile_mut(0, 1).set_notes_u8(&[7, 8]);
        board.get_tile_mut(0, 2).set_notes_u8(&[7, 8, 9]);

        resolve_single_notes(&mut board);
        assert_eq!(board.get_tile(0, 0).get_value(), Some(Number::Seven));
        assert_eq!(board.get_tile(0, 1).get_value(), None);
        assert_eq!(board.get_tile(0, 2).get_value(), None);
    }
}
