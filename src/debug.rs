//! Debugging utilities

use crate::board::{Board, ROW_SIZE};

/// FOR DEBUGGING ONLY
pub fn print_board(board: &Board) {
    const ROW_SEPARATOR: &str = "-------------------------";
    for y in 0..ROW_SIZE {
        if y % 3 == 0 {
            println!("{}", ROW_SEPARATOR);
        }
        let row: Vec<_> = board.get_row(y).collect();
        println!(
            "| {} {} {} | {} {} {} | {} {} {} |",
            row[0], // Comment added for better auto-formatting
            row[1],
            row[2],
            row[3],
            row[4],
            row[5],
            row[6],
            row[7],
            row[8],
        )
    }
    println!("{}", ROW_SEPARATOR);
}

pub fn print_notes(board: &Board) {
    println!("Print notes!");
    for y in 0..ROW_SIZE {
        for x in 0..ROW_SIZE {
            let tile = board.get_tile(x, y);
            if let Some(value) = tile.get_value() {
                println!("({}, {}): {}", x, y, value);
            } else {
                println!("({}, {}): {:?}", x, y, tile.get_notes_u8());
            }
        }
    }
}
