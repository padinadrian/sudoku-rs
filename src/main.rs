//! Main

/* ===== Includes ===== */

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use sudoku::Board;

/* ===== Main ===== */

/// Sub-command
#[derive(Debug, Subcommand)]
enum Command {
    /// Play a game
    Play { filename: PathBuf },

    /// Run the solver
    Solve { filename: PathBuf },
}

/// Command-line arguments
#[derive(Debug, Parser)]
struct Args {
    /// Subcommand
    #[command(subcommand)]
    command: Command,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // ratatui::run(app)?;

    let args = Args::parse();

    match args.command {
        Command::Play { filename: _ } => {
            // TODO
            color_eyre::Result::Ok(())
        }
        Command::Solve { filename } => {
            println!("Solving file: {:?}", filename.to_str());
            let mut board = Board::new();
            board.load_from_file(&filename).expect("Unable to solve");
            sudoku::solve::solve(&mut board)
        }
    }
}

#[cfg(test)]
mod tests {}
