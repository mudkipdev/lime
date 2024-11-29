use std::{fs::File, io::{BufReader, Error}, path::PathBuf};

use crossterm::event::KeyCode;
use ropey::Rope;

pub struct Selection {
    pub start: usize,
    pub end: usize
}

pub struct Buffer {
    pub text: Rope,
    pub file: Option<PathBuf>,
    pub position: usize,
    pub selection: Option<Selection>,
    pub modified: bool
}

impl Buffer {
    pub fn empty() -> Self {
        Self {
            text: Rope::new(),
            file: None,
            position: 0,
            selection: None,
            modified: false
        }
    }

    pub fn open(path: PathBuf) -> Result<Self, Error> {
        let file = File::open(path.clone())?;
        let reader = BufReader::new(file);
        let rope = Rope::from_reader(reader)?;

        Ok(Self {
            text: rope,
            file: Some(path),
            position: 0,
            selection: None,
            modified: false
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if self.file.is_none() {
            // TODO: cannot save non-file buffers
            return Ok(())
        }

        self.modified = false;
        Ok(())
    }

    pub fn get_line(&self) -> usize {
        self.text.char_to_line(self.position)
    }

    pub fn get_column(&self) -> usize {
        self.position - self.text.line_to_char(self.get_line())
    }

    pub fn handle_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Up => (),
            Movement::Left => {
                if self.position > 0 {
                    self.position -= 1;
                }
            },
            Movement::Right => self.position += 1,
            Movement::Down => (),
        }
    }

    pub fn insert(&mut self, text: &str) {
        self.text.insert(self.position, text);
        self.position += text.len();
        self.modified = true;
    }

    pub fn insert_character(&mut self, character: char) {
        self.text.insert_char(self.position, character);
        self.position += 1;
        self.modified = true;
    }

    pub fn delete_character(&mut self) {
        if self.position <= 0 {
            return;
        }

        self.text.remove(self.position - 1 .. self.position);
        self.position -= 1;
        self.modified = true;
    }
}

pub enum Movement {
    Up,
    Left,
    Right,
    Down
}

impl Movement {
    pub fn from_key_code(key_code: KeyCode) -> Option<Movement> {
        match key_code {
            KeyCode::Up => Option::Some(Movement::Up),
            KeyCode::Left => Option::Some(Movement::Left),
            KeyCode::Right => Option::Some(Movement::Right),
            KeyCode::Down => Option::Some(Movement::Down),
            _ => Option::None
        }
    }
}