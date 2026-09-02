//! Helper functions for solver: Check for obvious triples

use crate::board::{Board, ROW_SIZE, SquareIndexIter};

/// Check if the first set is a subset of the second set.
/// * `subset` the hopeful subset (smaller set)
/// * `superset` the superset to check (larger set)
fn is_subset<T>(subset: &[T], superset: &[T]) -> bool
where
    T: PartialEq,
{
    subset.iter().all(|n| superset.contains(n))
}

/// Check for obvious triples in the given row and clear notes from other tiles
/// if found.
///
/// Only process one set of triples per call.
///
fn check_obvious_triples_in_row(board: &mut Board, row: usize) -> bool {
    for column in 0..ROW_SIZE {
        // Triples can take two forms:
        // 1. All three tiles have the same 3 notes
        // 2. Two tiles have the same 3 notes and one tile has 2 matching notes
        // If two tiles have only 2 matching notes, that's a double not a triple
        let tile = board.get_tile(column, row);
        let notes = tile.get_notes();
        if notes.len() == 3 {
            // We need to start at the beginning of the row to look for a
            // matching subset of 2
            let mut matches = vec![];
            for column2 in 0..ROW_SIZE {
                if column2 != column {
                    let tile2 = board.get_tile(column2, row);
                    let notes2 = tile2.get_notes();
                    if notes2 == notes || (notes2.len() == 2 && is_subset(&notes2, &notes)) {
                        matches.push(column2)
                    }
                }
            }
            if matches.len() >= 2 {
                let mut changes_found = false;
                for column3 in 0..ROW_SIZE {
                    if column3 != column && !matches.contains(&column3) {
                        let tile3 = board.get_tile_mut(column3, row);
                        let notes_before = tile3.get_notes();
                        tile3.clear_notes(&notes);
                        changes_found |= tile3.get_notes() != notes_before;
                    }
                }
                if changes_found {
                    return true;
                }
            }
        }
    }
    false
}

/// Check for obvious triples in the given column and clear notes from other
/// tiles if found.
///
/// Only process one set of triples per call.
///
fn check_obvious_triples_in_column(board: &mut Board, column: usize) -> bool {
    for row in 0..ROW_SIZE {
        let tile = board.get_tile(column, row);
        let notes = tile.get_notes();
        if notes.len() == 3 {
            let mut matches = vec![];
            for row2 in 0..ROW_SIZE {
                if row2 != row {
                    let tile2 = board.get_tile(column, row2);
                    let notes2 = tile2.get_notes();
                    if notes2 == notes || (notes2.len() == 2 && is_subset(&notes2, &notes)) {
                        matches.push(row2);
                    }
                }
            }
            if matches.len() >= 2 {
                let mut changes_found = false;
                for row3 in 0..ROW_SIZE {
                    if row3 != row && !matches.contains(&row3) {
                        let tile3 = board.get_tile_mut(column, row3);
                        let notes_before = tile3.get_notes();
                        tile3.clear_notes(&notes);
                        changes_found |= tile3.get_notes() != notes_before;
                    }
                }
                if changes_found {
                    return true;
                }
            }
        }
    }
    false
}

/// Check for obvious triples in the given square and clear notes from other
/// tiles if found.
///
/// Only process one set of triples per call.
///
fn check_obvious_triples_in_square(board: &mut Board, square: usize) -> bool {
    for (column, row) in SquareIndexIter::new(square) {
        let tile = board.get_tile(column, row);
        let notes = tile.get_notes();
        if notes.len() == 3 {
            let mut matches = vec![];
            for (column2, row2) in SquareIndexIter::new(square) {
                if (column2, row2) != (column, row) {
                    let tile2 = board.get_tile(column2, row2);
                    let notes2 = tile2.get_notes();
                    if notes2 == notes || (notes2.len() == 2 && is_subset(&notes2, &notes)) {
                        matches.push((column2, row2));
                    }
                }
            }
            if matches.len() >= 2 {
                let mut changes_found = false;
                for (column3, row3) in SquareIndexIter::new(square) {
                    if (column3, row3) != (column, row) && !matches.contains(&(column3, row3)) {
                        let tile3 = board.get_tile_mut(column3, row3);
                        let notes_before = tile3.get_notes();
                        tile3.clear_notes(&notes);
                        changes_found |= tile3.get_notes() != notes_before;
                    }
                }
                if changes_found {
                    return true;
                }
            }
        }
    }
    false
}

pub fn check_obvious_triples(board: &mut Board) -> bool {
    for row in 0..ROW_SIZE {
        if check_obvious_triples_in_row(board, row) {
            return true;
        }
    }
    for column in 0..ROW_SIZE {
        if check_obvious_triples_in_column(board, column) {
            return true;
        }
    }
    for square in 0..ROW_SIZE {
        if check_obvious_triples_in_square(board, square) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::Number;

    #[test]
    fn test_is_subset() {
        assert!(is_subset(&[1, 2], &[1, 2, 3]));
        assert!(is_subset(&[3, 4, 5], &[3, 4, 5]));
        assert!(is_subset(&[1], &[1]));
        assert!(is_subset(&[], &[1]));
        assert!(is_subset::<u8>(&[], &[]));

        assert!(!is_subset(&[1, 2, 5], &[1, 2, 3]));
        assert!(!is_subset(&[3, 4, 5, 6], &[3, 4, 5]));
        assert!(!is_subset(&[3], &[4, 5]));
        assert!(!is_subset(&[1], &[]));
    }

    #[test]
    fn test_check_obvious_triples_in_row() {
        let mut board = Board::new();
        board.get_tile_mut(0, 0).set_notes_u8(&[1, 2, 5]);
        board.get_tile_mut(1, 0).set_notes_u8(&[1, 2]);
        board.get_tile_mut(2, 0).set_notes_u8(&[1, 2, 3]);
        board.get_tile_mut(3, 0).set_notes_u8(&[3, 4]);
        board.get_tile_mut(4, 0).set_notes_u8(&[6, 7, 8, 9]);
        board.get_tile_mut(5, 0).set_notes_u8(&[6, 7, 8]);
        board.get_tile_mut(6, 0).set_notes_u8(&[1, 2, 6, 7, 8]);
        board.get_tile_mut(7, 0).set_notes_u8(&[7, 8, 9]);
        board.get_tile_mut(8, 0).set_notes_u8(&[1, 2, 3]);

        assert!(check_obvious_triples_in_row(&mut board, 0));

        assert_eq!(board.get_tile(0, 0).get_notes(), vec![Number::Five]);
        assert_eq!(
            board.get_tile(1, 0).get_notes(),
            vec![Number::One, Number::Two]
        );
        assert_eq!(
            board.get_tile(2, 0).get_notes(),
            vec![Number::One, Number::Two, Number::Three]
        );
        assert_eq!(board.get_tile(3, 0).get_notes(), vec![Number::Four]);
        assert_eq!(
            board.get_tile(4, 0).get_notes(),
            vec![Number::Six, Number::Seven, Number::Eight, Number::Nine]
        );
        assert_eq!(
            board.get_tile(5, 0).get_notes(),
            vec![Number::Six, Number::Seven, Number::Eight]
        );
        assert_eq!(
            board.get_tile(6, 0).get_notes(),
            vec![Number::Six, Number::Seven, Number::Eight]
        );
        assert_eq!(
            board.get_tile(7, 0).get_notes(),
            vec![Number::Seven, Number::Eight, Number::Nine]
        );
        assert_eq!(
            board.get_tile(8, 0).get_notes(),
            vec![Number::One, Number::Two, Number::Three]
        );
    }

    #[test]
    fn test_check_obvious_triples_in_column() {
        let mut board = Board::new();
        board.get_tile_mut(0, 0).set_notes_u8(&[]);
        board.get_tile_mut(0, 1).set_notes_u8(&[4, 5, 6]);
        board.get_tile_mut(0, 2).set_notes_u8(&[1, 2, 3]);
        board.get_tile_mut(0, 3).set_notes_u8(&[4, 6]);
        board.get_tile_mut(0, 4).set_notes_u8(&[4]);
        board.get_tile_mut(0, 5).set_notes_u8(&[6, 7]);
        board.get_tile_mut(0, 6).set_notes_u8(&[6, 7, 8]);
        board.get_tile_mut(0, 7).set_notes_u8(&[4, 5, 6]);
        board.get_tile_mut(0, 8).set_notes_u8(&[7]);

        assert!(check_obvious_triples_in_column(&mut board, 0));

        assert_eq!(board.get_tile(0, 0).get_notes(), vec![]);
        assert_eq!(
            board.get_tile(0, 1).get_notes(),
            vec![Number::Four, Number::Five, Number::Six]
        );
        assert_eq!(
            board.get_tile(0, 2).get_notes(),
            vec![Number::One, Number::Two, Number::Three]
        );
        assert_eq!(
            board.get_tile(0, 3).get_notes(),
            vec![Number::Four, Number::Six]
        );
        assert_eq!(board.get_tile(0, 4).get_notes(), vec![]);
        assert_eq!(board.get_tile(0, 5).get_notes(), vec![Number::Seven]);
        assert_eq!(
            board.get_tile(0, 6).get_notes(),
            vec![Number::Seven, Number::Eight]
        );
        assert_eq!(
            board.get_tile(0, 7).get_notes(),
            vec![Number::Four, Number::Five, Number::Six]
        );
        assert_eq!(board.get_tile(0, 8).get_notes(), vec![Number::Seven]);
    }

    #[test]
    fn test_check_obvious_triples_in_square() {
        let mut board = Board::new();
        board.get_tile_mut(3, 3).set_notes_u8(&[5, 7, 8]);
        board.get_tile_mut(4, 3).set_notes_u8(&[4, 5, 6]);
        board.get_tile_mut(5, 3).set_notes_u8(&[]);
        board.get_tile_mut(3, 4).set_notes_u8(&[5, 7, 8]);
        board.get_tile_mut(4, 4).set_notes_u8(&[4, 5, 7, 8, 9]);
        board.get_tile_mut(5, 4).set_notes_u8(&[6, 7]);
        board.get_tile_mut(3, 5).set_notes_u8(&[6, 7, 8]);
        board.get_tile_mut(4, 5).set_notes_u8(&[1, 2, 3]);
        board.get_tile_mut(5, 5).set_notes_u8(&[7, 8]);

        assert!(check_obvious_triples_in_square(&mut board, 4));

        assert_eq!(
            board.get_tile(3, 3).get_notes(),
            vec![Number::Five, Number::Seven, Number::Eight]
        );
        assert_eq!(
            board.get_tile(4, 3).get_notes(),
            vec![Number::Four, Number::Six]
        );
        assert_eq!(board.get_tile(5, 3).get_notes(), vec![]);
        assert_eq!(
            board.get_tile(3, 4).get_notes(),
            vec![Number::Five, Number::Seven, Number::Eight]
        );
        assert_eq!(
            board.get_tile(4, 4).get_notes(),
            vec![Number::Four, Number::Nine]
        );
        assert_eq!(board.get_tile(5, 4).get_notes(), vec![Number::Six]);
        assert_eq!(board.get_tile(3, 5).get_notes(), vec![Number::Six]);
        assert_eq!(
            board.get_tile(4, 5).get_notes(),
            vec![Number::One, Number::Two, Number::Three]
        );
        assert_eq!(
            board.get_tile(5, 5).get_notes(),
            vec![Number::Seven, Number::Eight]
        );
    }
}
