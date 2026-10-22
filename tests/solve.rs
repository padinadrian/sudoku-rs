//! Integration tests for sudoku solver.

use std::path::PathBuf;

use sudoku::Board;

#[test]
fn test_easy1() {
    let filename = PathBuf::from("examples/easy/easy1.csv");
    let mut board = Board::new();
    board.load_from_file(&filename).unwrap();
    sudoku::solve::solve(&mut board).unwrap();
}

#[test]
fn test_medium1() {
    let filename = PathBuf::from("examples/medium/medium1.csv");
    let mut board = Board::new();
    board.load_from_file(&filename).unwrap();
    sudoku::solve::solve(&mut board).unwrap();
}

#[test]
fn test_hard1() {
    let filename = PathBuf::from("examples/hard/hard1.csv");
    let mut board = Board::new();
    board.load_from_file(&filename).unwrap();
    sudoku::solve::solve(&mut board).unwrap();
}

#[test]
fn test_expert1() {
    let filename = PathBuf::from("examples/expert/expert1.csv");
    let mut board = Board::new();
    board.load_from_file(&filename).unwrap();
    sudoku::solve::solve(&mut board).unwrap();
}

#[test]
fn test_master1() {
    let filename = PathBuf::from("examples/master/master1.csv");
    let mut board = Board::new();
    board.load_from_file(&filename).unwrap();
    sudoku::solve::solve(&mut board).unwrap();
}

#[test]
fn test_extreme1() {
    let filename = PathBuf::from("examples/extreme/extreme1.csv");
    let mut board = Board::new();
    board.load_from_file(&filename).unwrap();
    sudoku::solve::solve(&mut board).unwrap();
}

#[test]
fn test_random1() {
    let filename = PathBuf::from("examples/random/random1.csv");
    let mut board = Board::new();
    board.load_from_file(&filename).unwrap();
    sudoku::solve::solve(&mut board).unwrap();
}
