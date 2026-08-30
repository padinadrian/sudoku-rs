//! Main

/* ===== Includes ===== */

mod app;
mod board;
mod debug;
mod number;
mod serialize;
mod solve;

use board::Board;

/* ===== Main ===== */

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // ratatui::run(app)?;

    let filename = "examples/expert/expert1/expert1.csv";
    let mut board = Board::new();
    board.load_from_file(filename).unwrap();

    // crate::debug::print_board(&board);
    crate::solve::solve(&mut board)
}

#[cfg(test)]
mod tests {}
