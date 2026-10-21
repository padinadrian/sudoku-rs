//! Main

/* ===== Includes ===== */

use sudoku::Board;

/* ===== Main ===== */

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // ratatui::run(app)?;

    let filename = "examples/extreme/extreme1.csv";
    let mut board = Board::new();
    board.load_from_file(filename).unwrap();

    // crate::debug::print_board(&board);
    sudoku::solve::solve(&mut board)
}

#[cfg(test)]
mod tests {}
