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

#[derive(Debug, Subcommand)]
enum Command {
    /// Play a game (not yet implemented).
    Play { filename: PathBuf },

    /// Solve an existing puzzle from file.
    Solve { filename: PathBuf },
}

/// sudoku-rs - A sudoku solver and game player built in Rust.
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
