//! cli app

use crate::board::{Board, ROW_SIZE};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
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
pub struct App {
    board: Board,
    exit: bool,
}

impl App {
    /// Start a game from an existing game file.
    pub fn from_file(filename: &PathBuf) -> color_eyre::Result<Self> {
        let board = Board::from_file(filename)?;
        Ok(Self {
            board,
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
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            _ => {}
        }
    }

    /// Exit the app. Causes the run loop to terminate.
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Sudoku".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered());
        // .border_set(border::THICK);

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

        let grid = Grid::new(&self.board);
        grid.render(grid_layout[1], buf);
    }
}

struct Grid<'a> {
    board: &'a Board,
}

impl<'a> Grid<'a> {
    fn new(board: &'a Board) -> Self {
        Self { board }
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

        for (i, cell) in cells.enumerate() {
            let (row, col) = (i / ROW_SIZE, i % ROW_SIZE);
            Paragraph::new(format!("{}", self.board.get_tile(col, row)))
                .block(Block::bordered().merge_borders(MergeStrategy::Exact))
                .centered()
                .render(cell, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_key_event_quit() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }

    #[test]
    fn test_handle_key_event_ctrl_c() {
        let mut app = App::default();
        app.handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        assert!(app.exit);
    }
}
