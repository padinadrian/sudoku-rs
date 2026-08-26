//! Main

/* ===== Includes ===== */

mod app;
mod board;
mod debug;
mod number;
mod serialize;
mod solve;

use board::Board;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // ratatui::run(app)?;

    let mut board = Board::default();
    board.load_from_file("examples/easy.csv").unwrap();

    // crate::debug::print_board(&board);
    crate::solve::solve(&mut board)
}

#[cfg(test)]
mod tests {}
