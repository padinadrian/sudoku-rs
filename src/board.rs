//! Board class

use crate::number::Number;

const ROW_SIZE: usize = 9;
const BOARD_SIZE: usize = ROW_SIZE * ROW_SIZE;

const ALL_NUMBERS: [Number; ROW_SIZE] = [
    Number::One,
    Number::Two,
    Number::Three,
    Number::Four,
    Number::Five,
    Number::Six,
    Number::Seven,
    Number::Eight,
    Number::Nine,
];

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tile {
    pub notes: [Number; ROW_SIZE],
    pub value: Option<Number>,
}

impl Tile {
    /// Construct a new tile with a given value.
    pub fn new(value: Number) -> Self {
        Self {
            value: Some(value),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Row {
    pub tiles: [Tile; ROW_SIZE],
}

impl Row {
    /// Construct a new Row from a given list of values.
    pub fn new(values: &[Number; ROW_SIZE]) -> Self {
        Self {
            tiles: values.map(Tile::new),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Board {
    pub rows: [Row; ROW_SIZE],
}

type SerializedRow = [u8; ROW_SIZE];

/// Convert an x/y coordinate to an array index.
fn to_index(x: usize, y: usize) -> usize {
    return x + (y * ROW_SIZE);
}

/// Check if the row has exactly one of each sudoku value.
fn row_is_solved(row: &Row) -> bool {
    let any_blank = row.tiles.iter().any(|tile| tile.value.is_none());
    if any_blank {
        return false;
    }

    // Unwrap is safe because of the above check.
    let values: Vec<Number> = row
        .tiles
        .iter()
        .map(|tile| tile.value.expect("Tile value should not be blank!"))
        .collect();

    values.contains(&Number::One)
        && values.contains(&Number::Two)
        && values.contains(&Number::Three)
        && values.contains(&Number::Four)
        && values.contains(&Number::Five)
        && values.contains(&Number::Six)
        && values.contains(&Number::Seven)
        && values.contains(&Number::Eight)
        && values.contains(&Number::Nine)
}

impl Board {
    /// Get a ref to the tile at the given coordinates.
    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.rows[y].tiles[x]
    }

    /// Get a mutable ref to the tile at the given coordinates.
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.rows[y].tiles[x]
    }

    /// Get value of the tile at the given coordinates, if it exists.
    pub fn get_value(&self, x: usize, y: usize) -> Option<Number> {
        self.get_tile(x, y).value
    }

    /// Get a single row of the board.
    pub fn get_row(&self, n: usize) -> Option<Row> {
        if n >= ROW_SIZE {
            return None;
        }
        Some(self.rows[n].clone())
    }

    /// Get a single column of the board.
    pub fn get_column(&self, n: usize) -> Option<Row> {
        if n >= ROW_SIZE {
            return None;
        }
        let mut col = Row::default();
        for y in 0..ROW_SIZE {
            col.tiles[y] = self.rows[y].tiles[n].clone();
        }
        Some(col)
    }

    /// Get a 3x3 square of the board.
    /// TODO: Unit tests for this function.
    pub fn get_square(&self, n: usize) -> Option<Row> {
        if n >= ROW_SIZE {
            return None;
        }
        let mut square = Row::default();
        let x_start = (n % 3) * 3;
        let y_start = (n / 3) * 3;

        // TODO: Use an iterator for this
        square.tiles[0] = self.rows[y_start + 0].tiles[x_start + 0].clone();
        square.tiles[1] = self.rows[y_start + 0].tiles[x_start + 1].clone();
        square.tiles[2] = self.rows[y_start + 0].tiles[x_start + 2].clone();

        square.tiles[3] = self.rows[y_start + 1].tiles[x_start + 0].clone();
        square.tiles[4] = self.rows[y_start + 1].tiles[x_start + 1].clone();
        square.tiles[5] = self.rows[y_start + 1].tiles[x_start + 2].clone();

        square.tiles[6] = self.rows[y_start + 2].tiles[x_start + 0].clone();
        square.tiles[7] = self.rows[y_start + 2].tiles[x_start + 1].clone();
        square.tiles[8] = self.rows[y_start + 2].tiles[x_start + 2].clone();

        Some(square)
    }

    /// Set the value of the tile at the given coordinates.
    pub fn set(&mut self, x: usize, y: usize, value: Option<Number>) {
        self.rows[y].tiles[x].value = value;
    }

    /// Load a saved game from a CSV file.
    pub fn load_from_file(&mut self, path: &str) -> color_eyre::Result<()> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .from_path(path)?;

        for (y, result) in reader.deserialize().enumerate() {
            let row: SerializedRow = result?;
            for x in 0..ROW_SIZE {
                self.set(x, y, Some(Number::from(row[x])));
            }
        }
        Ok(())
    }

    pub fn load_from_str(&mut self, serialized: &str) -> color_eyre::Result<()> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .from_reader(serialized.as_bytes());

        for (y, result) in reader.deserialize().enumerate() {
            let row: SerializedRow = result?;
            for x in 0..ROW_SIZE {
                self.set(x, y, Some(Number::from(row[x])));
            }
        }
        Ok(())
    }

    /// Check if the board is solved and the game is finished.
    pub fn is_solved(&self) -> bool {
        for n in 0..ROW_SIZE {
            if !row_is_solved(
                &self
                    .get_row(n)
                    .expect(&format!("Row index {} out of bounds", n)),
            ) {
                return false;
            }
        }
        for n in 0..ROW_SIZE {
            if !row_is_solved(
                &self
                    .get_column(n)
                    .expect(&format!("Row index {} out of bounds", n)),
            ) {
                return false;
            }
        }
        for n in 0..ROW_SIZE {
            if !row_is_solved(
                &self
                    .get_square(n)
                    .expect(&format!("Row index {} out of bounds", n)),
            ) {
                return false;
            }
        }
        true
    }

    /// Fill every unsolved space with notes for every value.
    pub fn fill_all_notes(&mut self) {
        for y in 0..ROW_SIZE {
            for x in 0..ROW_SIZE {
                let tile = self.get_tile_mut(x, y);
                if tile.value.is_none() {
                    tile.notes = ALL_NUMBERS.clone();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOLVED_BOARD: &str = "1,2,3,4,5,6,7,8,9
4,5,6,7,8,9,1,2,3
7,8,9,1,2,3,4,5,6
2,3,1,5,6,4,8,9,7
5,6,4,8,9,7,2,3,1
8,9,7,2,3,1,5,6,4
3,1,2,6,4,5,9,7,8
6,4,5,9,7,8,3,1,2
9,7,8,3,1,2,6,4,5";

    const UNSOLVED_BOARD: &str = "1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9
1,2,3,4,5,6,7,8,9";

    #[test]
    fn test_board_get_row() {
        let mut board = Board::default();
        board.load_from_str(SOLVED_BOARD).unwrap();
        let expected_ints = [2, 3, 1, 5, 6, 4, 8, 9, 7];
        let expected_tiles = expected_ints.map(|n| Tile::new(Number::from(n)));
        assert_eq!(board.get_row(3).unwrap().tiles, expected_tiles);
    }

    #[test]
    fn test_board_get_column() {
        let mut board = Board::default();
        board.load_from_str(SOLVED_BOARD).unwrap();
        let expected_ints = [5, 8, 2, 6, 9, 3, 4, 7, 1];
        let expected_tiles = expected_ints.map(|n| Tile::new(Number::from(n)));
        assert_eq!(board.get_column(4).unwrap().tiles, expected_tiles);
    }

    #[test]
    fn test_board_get_square() {
        let mut board = Board::default();
        board.load_from_str(SOLVED_BOARD).unwrap();
        let expected_ints = [6, 4, 5, 9, 7, 8, 3, 1, 2];
        let expected_tiles = expected_ints.map(|n| Tile::new(Number::from(n)));
        assert_eq!(board.get_square(7).unwrap().tiles, expected_tiles);
    }

    #[test]
    fn test_board_is_solved() {
        let mut board = Board::default();
        board.load_from_str(SOLVED_BOARD).unwrap();
        assert!(board.is_solved());
    }

    #[test]
    fn test_board_is_not_solved() {
        let mut board = Board::default();
        board.load_from_str(UNSOLVED_BOARD).unwrap();
        assert!(!board.is_solved());
    }

    #[test]
    fn test_board_fill_all_notes() {
        let mut board = Board::default();
        board.get_tile_mut(3, 5).value = Some(Number::Five);
        board.fill_all_notes();
        assert_eq!(board.get_value(3, 5), Some(Number::Five));
        assert_eq!(board.get_value(3, 4), None);
        assert_eq!(board.get_tile(3, 5).notes, [Number::default(); ROW_SIZE]);
        assert_eq!(board.get_tile(3, 4).notes, ALL_NUMBERS);
    }
}
