//! Grid Widget class (v2)

use crate::app::ascii_art::ASCII_NUMBER_LOOKUP;
use crate::board::{Board, ROW_SIZE};
use crate::number::Number;
use crate::tile::Tile;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    symbols::{border, merge::MergeStrategy},
    widgets::{Block, Paragraph, Widget},
};

pub const TILES_PER_ROW: u16 = ROW_SIZE as u16;
pub const TILE_WIDTH: u16 = 11;
pub const TILE_HEIGHT: u16 = 5;

fn format_notes(tile: &Tile) -> String {
    format!(
        "{} {} {}\n{} {} {}\n{} {} {}",
        if tile.has_note(Number::One) { "1" } else { " " },
        if tile.has_note(Number::Two) { "2" } else { " " },
        if tile.has_note(Number::Three) {
            "3"
        } else {
            " "
        },
        if tile.has_note(Number::Four) {
            "4"
        } else {
            " "
        },
        if tile.has_note(Number::Five) {
            "5"
        } else {
            " "
        },
        if tile.has_note(Number::Six) { "6" } else { " " },
        if tile.has_note(Number::Seven) {
            "7"
        } else {
            " "
        },
        if tile.has_note(Number::Eight) {
            "8"
        } else {
            " "
        },
        if tile.has_note(Number::Nine) {
            "9"
        } else {
            " "
        },
    )
}

#[derive(Debug, Default)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

pub struct Grid<'a> {
    board: &'a Board,
    cursor: &'a Cursor,
}

impl<'a> Grid<'a> {
    /// Create a new Grid widget
    pub fn new(board: &'a Board, cursor: &'a Cursor) -> Self {
        Self { board, cursor }
    }
}

impl<'a> Widget for &Grid<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let col_constraints = (0..9).map(|_| Constraint::Length(TILE_WIDTH));
        let row_constraints = (0..9).map(|_| Constraint::Length(TILE_HEIGHT));
        let horizontal = Layout::horizontal(col_constraints).spacing(-1);
        let vertical = Layout::vertical(row_constraints).spacing(-1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|row| row.layout_vec(&horizontal));

        let cursor_value = self
            .board
            .get_tile(self.cursor.x, self.cursor.y)
            .get_value();

        for (i, cell) in cells.enumerate() {
            let (row, col) = (i / ROW_SIZE, i % ROW_SIZE);
            let tile = self.board.get_tile(col, row);
            let mut style = Style::new();
            if tile.is_locked() {
                style = style.bold();
            }
            if (row == self.cursor.y) || (col == self.cursor.x) {
                style = style.bg(Color::Blue);
            }
            if cursor_value.is_some() && cursor_value == tile.get_value() {
                let mut mistake = false;
                if (row, col) == (self.cursor.y, self.cursor.x) {
                    for (i, t) in self.board.get_row(row).enumerate() {
                        if i != col && t.get_value() == cursor_value {
                            mistake = true;
                            break;
                        }
                    }
                    for (i, t) in self.board.get_column(col).enumerate() {
                        if i != row && t.get_value() == cursor_value {
                            mistake = true;
                            break;
                        }
                    }
                }
                if mistake {
                    style = style.bg(Color::LightRed);
                } else {
                    style = style.bg(Color::Green);
                }
            }

            let display_str = match tile.get_value() {
                None => format_notes(tile),
                Some(value) => String::from(ASCII_NUMBER_LOOKUP[value as usize]),
            };
            Paragraph::new(format!("{}", display_str))
                .block(Block::bordered().merge_borders(MergeStrategy::Exact))
                .centered()
                .style(style)
                .render(cell, buf);
        }

        let col_constraints2 = (0..3).map(|_| Constraint::Length(TILE_WIDTH * 3 - 2));
        let row_constraints2 = (0..3).map(|_| Constraint::Length(TILE_HEIGHT * 3 - 2));
        let horizontal2 = Layout::horizontal(col_constraints2).spacing(-1);
        let vertical2 = Layout::vertical(row_constraints2).spacing(-1);

        let square_rows = vertical2.split(area);
        let squares = square_rows
            .iter()
            .flat_map(|row| row.layout_vec(&horizontal2));
        for square in squares {
            Paragraph::new("")
                .block(
                    Block::bordered()
                        .border_set(border::THICK)
                        .merge_borders(MergeStrategy::Exact),
                )
                .centered()
                .render(square, buf);
        }
    }
}
