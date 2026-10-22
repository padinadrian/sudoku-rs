//! Sudoku player and solver.
//!
//! This program has two modes:
//! * `play` - Play a game
//! * `solve` - Solve an existing puzzle
//!

/* ===== Includes ===== */

use clap::{Parser, Subcommand};
use eyre;
use std::path::PathBuf;
use sudoku::Board;

/* ===== Command-line Arguments ===== */

/// Sub-command
#[derive(Debug, Subcommand)]
enum Command {
    /// Play a game.
    Play { filename: PathBuf },

    /// Solve an existing puzzle from file.
    Solve { filename: PathBuf },
}

/// Command-line arguments
#[derive(Debug, Parser)]
struct Args {
    /// Subcommand
    #[command(subcommand)]
    command: Command,
}

/* ===== Main ===== */

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    match args.command {
        Command::Play { filename: _ } => {
            // TODO
            // ratatui::run(app)?;
            color_eyre::Result::Ok(())
        }
        Command::Solve { filename } => {
            if !filename.is_file() {
                return Err(eyre::eyre!("Provided path was not a file: {:?}", filename));
            }
            println!("Solving file: {:?}", filename);
            let mut board = Board::new();
            board.load_from_file(&filename).expect("Unable to solve");
            sudoku::solve::solve(&mut board)
        }
    }
}
