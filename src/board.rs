//! Board class

use crate::number::Number;

const ROW_SIZE: usize = 9;
const BOARD_SIZE: usize = ROW_SIZE * ROW_SIZE;

#[derive(Debug, Default, Clone)]
pub struct Tile {
    pub notes: [Number; ROW_SIZE],
    pub value: Option<Number>,
}

#[derive(Debug, Default, Clone)]
pub struct Row {
    pub tiles: [Tile; ROW_SIZE],
}

#[derive(Debug, Default, Clone)]
pub struct Board {
    pub rows: [Row; ROW_SIZE],
}

type SerializedRow = [u8; ROW_SIZE];

fn to_index(x: usize, y: usize) -> usize {
    return x + (y * ROW_SIZE);
}

impl Board {
    /// Get value of the tile at the given coordinates, if it exists.
    pub fn get(&self, x: usize, y: usize) -> Option<Number> {
        return self.rows[y].tiles[x].value;
    }

    /// Set the value of the tile at the given coordinates.
    pub fn set(&mut self, x: usize, y: usize, value: Option<Number>) {
        self.rows[y].tiles[x].value = value;
    }

    /// Load a saved game from a CSV file.
    pub fn load_game(&mut self, path: &str) -> color_eyre::Result<()> {
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
        return Ok(());
    }
}
