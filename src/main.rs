//! Sudoku player and solver.
//!
//! This program has two modes:
//! * `play` - Play a game
//! * `solve` - Solve an existing puzzle
//!

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
#[command(version)]
struct Args {
    /// Subcommand
    #[command(subcommand)]
    command: Option<Command>,
}

/* ===== Main ===== */

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            Command::Play { filename: _ } => {
                // TODO
                // ratatui::run(app)?;
                Ok(())
            }
            Command::Solve { filename } => {
                if !filename.is_file() {
                    return Err(eyre::eyre!("Provided path was not a file: {:?}", filename));
                }
                println!("Solving file: {:?}", filename);
                let mut board = Board::from_file(&filename)?;
                sudoku::solve::solve(&mut board)
            }
        }
    } else {
        Ok(())
    }
}
