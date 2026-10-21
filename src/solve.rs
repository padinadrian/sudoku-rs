//! Main entry point to solver

mod obvious_doubles;
mod obvious_triples;
mod only_choice;
mod sweeping_lines;

use crate::board::{Board, ROW_SIZE, SquareIndexIter, to_square_index};
use crate::debug::{print_board, print_notes};
use crate::number::Number;
use crate::solve::sweeping_lines::check_sweeping_lines;
use obvious_doubles::check_obvious_doubles;
use obvious_triples::check_obvious_triples;
use only_choice::resolve_only_choice_remaining;

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
        println!("\n");
        println!("Iteration: {}", counter);
        counter += 1;

        // TODO: Debugging only, remove later
        // TODO: Debug function to print board with all notes
        print_board(board);
        print_notes(board);

        // Always do this step because it doesn't set any values, it only
        // changes the notes.
        simplify_notes(board);

        // Only do one of these at a time so we can keep track of the steps
        // more easily.
        if resolve_single_notes(board) {
            println!("Resolved single note.");
            continue;
        }
        if resolve_only_choice_remaining(board) {
            println!("Resolved only choice remaining.");
            continue;
        }
        if check_obvious_doubles(board) {
            println!("Obvious double found.");
            continue;
        }
        if check_obvious_triples(board) {
            println!("Obvious triple found.");
            continue;
        }
        if check_sweeping_lines(board) {
            println!("Sweeping line found.");
            continue;
        }

        // Solver steps TODO:
        //  * Find hidden doubles
        //  * Find hidden triples
        //  * X-ray
        //  * Bowman's bingo

        if prev_board == *board {
            print_board(board);
            print_notes(board);
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
}
