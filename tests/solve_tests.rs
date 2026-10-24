//! Integration tests for sudoku solver.

use std::path::PathBuf;
use sudoku::Board;

fn run_solve_test(starting_path: &str, solution_path: &str) {
    let start_path = PathBuf::from(starting_path);
    let solution_path = PathBuf::from(solution_path);
    let mut board = Board::from_file(&start_path).expect("failed to read file");
    let solution = Board::from_file(&solution_path).expect("failed to read file");
    sudoku::solve::solve(&mut board).unwrap();
    assert_eq!(board, solution);
}

#[test]
fn test_easy1() {
    run_solve_test("examples/easy/easy1.csv", "examples/easy/easy1_solved.csv");
}

#[test]
fn test_medium1() {
    run_solve_test(
        "examples/medium/medium1.csv",
        "examples/medium/medium1_solved.csv",
    );
}

#[test]
fn test_hard1() {
    run_solve_test("examples/hard/hard1.csv", "examples/hard/hard1_solved.csv");
}

#[test]
fn test_expert1() {
    run_solve_test(
        "examples/expert/expert1.csv",
        "examples/expert/expert1_solved.csv",
    );
}

#[test]
fn test_master1() {
    run_solve_test(
        "examples/master/master1.csv",
        "examples/master/master1_solved.csv",
    );
}

#[test]
fn test_extreme1() {
    run_solve_test(
        "examples/extreme/extreme1.csv",
        "examples/extreme/extreme1_solved.csv",
    );
}

#[test]
fn test_extreme2() {
    run_solve_test(
        "examples/extreme/extreme2.csv",
        "examples/extreme/extreme2_solved.csv",
    );
}

#[test]
fn test_random1() {
    run_solve_test(
        "examples/random/random1.csv",
        "examples/random/random1_solved.csv",
    );
}
