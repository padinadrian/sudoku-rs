//! Main

/* ===== Includes ===== */

mod app;
mod board;
mod number;
mod serialize;

use board::Board;

/// FOR DEBUGGING ONLY
fn print_board(board: &Board) {
    const ROW_SEPARATOR: &str = "-------------------------";
    for (y, row) in board.rows.iter().enumerate() {
        if y % 3 == 0 {
            println!("{}", ROW_SEPARATOR);
        }
        println!(
            "| {} {} {} | {} {} {} | {} {} {} |",
            row.tiles[0].value.unwrap_or_default(),
            row.tiles[1].value.unwrap_or_default(),
            row.tiles[2].value.unwrap_or_default(),
            row.tiles[3].value.unwrap_or_default(),
            row.tiles[4].value.unwrap_or_default(),
            row.tiles[5].value.unwrap_or_default(),
            row.tiles[6].value.unwrap_or_default(),
            row.tiles[7].value.unwrap_or_default(),
            row.tiles[8].value.unwrap_or_default(),
        )
    }
    println!("{}", ROW_SEPARATOR);
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // ratatui::run(app)?;

    let mut board = Board::default();
    board.load_game("./game.csv").unwrap();

    print_board(&board);

    Ok(())
}

#[cfg(test)]
mod tests {}
