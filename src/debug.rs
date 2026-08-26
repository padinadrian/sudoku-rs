//! Debugging utilities

use crate::board::{Board, ROW_SIZE};

/// FOR DEBUGGING ONLY
pub fn print_board(board: &Board) {
    const ROW_SEPARATOR: &str = "-------------------------";
    for (y, row) in board.rows.iter().enumerate() {
        if y % 3 == 0 {
            println!("{}", ROW_SEPARATOR);
        }
        println!(
            "| {} {} {} | {} {} {} | {} {} {} |",
            row.tiles[0],
            row.tiles[1],
            row.tiles[2],
            row.tiles[3],
            row.tiles[4],
            row.tiles[5],
            row.tiles[6],
            row.tiles[7],
            row.tiles[8],
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
                println!("({}, {}): {:?}", x, y, tile.get_notes());
            }
        }
    }
}
