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
    Solve {
        filename: PathBuf,
        /// Optional output file to save result
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

/// sudoku-rs - Sudoku solver and game player, built in Rust.
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
            Command::Solve { filename, output } => {
                if !filename.is_file() {
                    return Err(eyre::eyre!("Provided path was not a file: {:?}", filename));
                }
                println!("Solving file: {:?}", filename);
                let mut board = Board::from_file(&filename)?;
                let result = sudoku::solve::solve(&mut board);
                if result.is_ok() {
                    if let Some(output_file) = output {
                        board.save_to_file(&output_file)?;
                    }
                }
                result
            }
        }
    } else {
        Ok(())
    }
}
