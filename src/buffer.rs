use std::{fs::File, io::{BufReader, Error}};

use crossterm::event::KeyCode;
use ropey::Rope;

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

pub struct Buffer {
    pub text: Rope,
    pub position: usize,
    pub modified: bool
}

impl Buffer {
    pub fn empty() -> Self {
        Self {
            text: Rope::new(),
            position: 0,
            modified: false
        }
    }

    pub fn open(path: &str) -> Result<Self, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let rope = Rope::from_reader(reader)?;

        Ok(Self {
            text: rope,
            position: 0,
            modified: false
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
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