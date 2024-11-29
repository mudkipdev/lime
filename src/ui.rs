use std::{io::{self, Write}, panic};
use crossterm::{cursor, event, execute, queue, style::{self, Color}, terminal};

use crate::{buffer::Buffer, editor::Editor};

const CELL: Cell = Cell {
    background: Color::Black,
    foreground: Color::White,
    content: ' ',
};

#[derive(Clone, Copy)]
pub struct Cell {
    pub background: Color,
    pub foreground: Color,
    pub content: char
}

pub struct Terminal {
    cells: Vec<Cell>,
    pub columns: u16,
    pub rows: u16
}

pub fn enter_raw_mode() {
    terminal::enable_raw_mode().unwrap();

    execute!(
        io::stdout(),
        terminal::EnterAlternateScreen,
        event::EnableBracketedPaste,
        event::EnableFocusChange,
        event::EnableMouseCapture,
        cursor::Hide
    ).unwrap();
}

pub fn exit_raw_mode() {
    execute!(
        io::stdout(),
        cursor::Show,
        event::DisableBracketedPaste,
        event::DisableFocusChange,
        event::DisableMouseCapture,
        terminal::LeaveAlternateScreen
    ).unwrap();

    terminal::disable_raw_mode().unwrap();
}

pub trait Canvas {
    fn width(&self) -> u16;

    fn height(&self) -> u16;

    fn get(&self, x: u16, y: u16) -> Option<&Cell>;

    fn set(&mut self, x: u16, y: u16, cell: Cell);
}

impl Terminal {
    pub fn new() -> Terminal {
        let size = terminal::window_size().unwrap();

        Self {
            cells: vec![CELL; (size.columns * size.rows) as usize],
            columns: size.columns,
            rows: size.rows
        }
    }

    pub fn set_panic_hook(&mut self) {
        let default = panic::take_hook();

        panic::set_hook(Box::new(move |info| {
            exit_raw_mode();
            default(info);
        }));
    }

    pub fn render(&self) {
        queue!(
            io::stdout(),
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::All)
        ).unwrap();

        for x in 0 .. self.columns {
            for y in 0 .. self.rows {
                let cell = self.get(x, y).or(Some(&CELL)).unwrap();

                queue!(
                    io::stdout(),
                    cursor::MoveTo(x, y),
                    style::SetBackgroundColor(cell.background),
                    style::SetForegroundColor(cell.foreground),
                    style::Print(cell.content)
                ).unwrap();
            }
        }

        io::stdout().flush().unwrap();
    }
}

impl Canvas for Terminal {
    fn width(&self) -> u16 {
        return self.columns;
    }

    fn height(&self) -> u16 {
        return self.rows;
    }

    fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        if x < self.columns && y < self.rows {
            let index = (y * self.columns + x) as usize;
            self.cells.get(index)
        } else {
            None
        }
    }

    fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.columns && y < self.rows {
            let index = (y * self.columns + x) as usize;
            self.cells[index] = cell;
        }
    }
}

pub trait Element {
    fn render<T: Canvas>(&self, canvas: &mut T);
}

pub struct StatusBar<'a> {
    pub buffer: &'a Buffer
}

impl Element for StatusBar<'_> {
    fn render<T: Canvas>(&self, canvas: &mut T) {
        let cell = Cell {
            background: Color::White,
            foreground: Color::Black,
            content: ' '
        };

        for x in 0 .. canvas.width() {
            canvas.set(x, canvas.height() - 1, cell);
        }

        let string = format!(
            "({}, {})",
            self.buffer.get_line(),
            self.buffer.get_column()
        );

        for (index, character) in string.chars().enumerate() {
            canvas.set(index as u16, canvas.height() - 1, Cell {
                background: Color::White,
                foreground: Color::Black,
                content: character
            });
        }
    }
}