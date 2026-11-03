//! cli app

use crate::{
    board::{Board, ROW_SIZE},
    number::Number,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::{border, merge::MergeStrategy},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use std::{io, path::PathBuf, vec};

const TILES_PER_ROW: u16 = ROW_SIZE as u16;
const TILE_WIDTH: u16 = 6;
const TILE_HEIGHT: u16 = 3;
const GRID_WIDTH: u16 = TILES_PER_ROW * TILE_WIDTH;
const GRID_HEIGHT: u16 = TILES_PER_ROW * TILE_HEIGHT;

const MIN_WINDOW_WIDTH: u16 = 56;
const MIN_WINDOW_HEIGHT: u16 = 21;

#[derive(Debug, Default)]
struct Cursor {
    x: usize,
    y: usize,
}

#[derive(Debug, Default)]
pub struct App {
    board: Board,
    cursor: Cursor,
    solved: bool,
    exit: bool,
}

impl App {
    /// Start a game from an existing game file.
    pub fn from_file(filename: &PathBuf) -> color_eyre::Result<Self> {
        let board = Board::from_file(filename, true)?;
        let solved = board.is_solved();
        Ok(Self {
            board,
            solved,
            ..Default::default()
        })
    }

    /// Run the terminal app until it is terminated.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Render the app in the given window frame.
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// Handle events such as key presses.
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    /// Handle a key press event.
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::Up => {
                if self.cursor.y > 0 {
                    self.cursor.y -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor.y < 8 {
                    self.cursor.y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor.x > 0 {
                    self.cursor.x -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor.x < 8 {
                    self.cursor.x += 1;
                }
            }
            KeyCode::Char('1') => self.handle_number_input(Number::One),
            KeyCode::Char('2') => self.handle_number_input(Number::Two),
            KeyCode::Char('3') => self.handle_number_input(Number::Three),
            KeyCode::Char('4') => self.handle_number_input(Number::Four),
            KeyCode::Char('5') => self.handle_number_input(Number::Five),
            KeyCode::Char('6') => self.handle_number_input(Number::Six),
            KeyCode::Char('7') => self.handle_number_input(Number::Seven),
            KeyCode::Char('8') => self.handle_number_input(Number::Eight),
            KeyCode::Char('9') => self.handle_number_input(Number::Nine),
            KeyCode::Backspace | KeyCode::Delete => self.handle_clear_value(),
            _ => {}
        }
    }

    /// Handle a number input from the board UI.
    fn handle_number_input(&mut self, value: Number) {
        if !self.solved {
            self.board.set(self.cursor.x, self.cursor.y, Some(value));
            if self.board.is_solved() {
                self.solved = true;
            }
        }
    }

    /// Handle clearing the currently selected tile.
    fn handle_clear_value(&mut self) {
        if !self.solved {
            self.board.set(self.cursor.x, self.cursor.y, None);
        }
    }

    /// Exit the app. Causes the run loop to terminate.
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Sudoku ".bold());
        let block = if self.solved {
            let solved_text: Line = Line::from(vec![
                " Solved! Press ".into(),
                "<Ctrl-C>".blue().bold(),
                " to Quit ".into(),
            ]);
            Block::bordered()
                .title(title.centered())
                .title_bottom(solved_text.centered())
        } else {
            let instructions = Line::from(vec![
                " Cursor: ".into(),
                "← → ↑ ↓".blue().bold(),
                format!(" ({}, {})", self.cursor.x + 1, self.cursor.y + 1).into(),
                " | Quit: ".into(),
                "<Ctrl-C> ".blue().bold(),
            ]);
            Block::bordered()
                .title(title.centered())
                .title_bottom(instructions.centered())
        };

        block.render(area, buf);

        if area.width < MIN_WINDOW_WIDTH || area.height < MIN_WINDOW_HEIGHT {
            let warning_area = area.centered(Constraint::Length(55), Constraint::Length(3));
            let warning_block = Block::bordered().border_set(border::THICK);

            let warning_text = format!(
                "Please resize the window to at least {} by {}.",
                MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT
            );
            Paragraph::new(warning_text.yellow())
                .centered()
                .block(warning_block)
                .render(warning_area, buf);
            return;
        }

        let grid_area = area.centered(
            Constraint::Length(GRID_WIDTH),
            Constraint::Length(GRID_HEIGHT + 2),
        );

        let grid_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Length(GRID_HEIGHT),
                Constraint::Length(1),
            ])
            .split(grid_area);

        let grid = Grid::new(&self.board, &self.cursor);
        grid.render(grid_layout[1], buf);
    }
}

struct Grid<'a> {
    board: &'a Board,
    cursor: &'a Cursor,
}

impl<'a> Grid<'a> {
    fn new(board: &'a Board, cursor: &'a Cursor) -> Self {
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
                style = style.bg(Color::Cyan);
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
                    style = style.bg(Color::LightGreen);
                }
            }

            Paragraph::new(format!("{}", tile))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_key_event_ctrl_c() {
        let mut app = App::default();
        app.handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        assert!(app.exit);
    }
}
