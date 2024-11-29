use std::{panic, process::exit};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::{buffer::{Buffer, Movement}, config::Config, theme::ThemeManager, ui::{enter_raw_mode, exit_raw_mode, StatusBar, Terminal, Element}};

pub struct Editor {
    pub config: Config,
    pub terminal: Terminal,
    pub buffer: Buffer,
    pub theme_manager: ThemeManager,
    pub status_bar: StatusBar<'static>
}

impl Editor {
    pub fn new() -> Editor {
        let config = match Config::load() {
            Ok(mut config) => config.get_or_insert_with(Config::new).clone(),
            Err(_) => panic!("Failed to load config!")
        };

        match config.save() {
            Err(_) => panic!("Failed to save config!"),
            _ => ()
        };

        let buffer = Buffer::empty();

        let editor = Self {
            config: config,
            terminal: Terminal::new(),
            buffer: buffer,
            theme_manager: ThemeManager::new(),
            status_bar: StatusBar { buffer: &buffer }
        };

        editor
    }

    pub fn initialize(&mut self) {
        self.terminal.set_panic_hook();
        enter_raw_mode();
    }

    pub fn quit(&mut self) {
        exit_raw_mode();
        exit(0);
    }

    pub fn handle_events(&mut self) {
        match event::read().unwrap() {
            Event::Resize(columns, rows) => {
                self.terminal.columns = columns;
                self.terminal.rows = rows;
            },
            Event::Key(event) => {
                if event.modifiers == KeyModifiers::CONTROL {
                    match event.code {
                        KeyCode::Char('q') | KeyCode::Char('c') => self.quit(),
                        KeyCode::Char(' ') => self.theme_manager.next_theme(),
                        _ => ()
                    }
                } else if event.modifiers == KeyModifiers::NONE || event.modifiers == KeyModifiers::SHIFT {
                    match event.code {
                        KeyCode::Char(character) => self.buffer.insert_character(character),
                        KeyCode::Backspace => self.buffer.delete_character(),
                        _ => ()
                    }

                    if let Some(movement) = Movement::from_key_code(event.code) {
                        self.buffer.handle_movement(movement);
                    }
                }
            },
            _ => ()
        }
    }

    pub fn render(&mut self) {
        self.terminal.render();
        self.status_bar.render(&mut self.terminal);
    }
}