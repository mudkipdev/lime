use std::{io::{self, Error}, panic, process::exit};
use crossterm::{
    execute,
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Print, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}
};

use crate::{buffer::{Buffer, Movement}, config::Config, theme::ThemeManager};

pub struct Editor {
    pub config: Config,
    pub buffer: Buffer,
    pub theme_manager: ThemeManager
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

        Self {
            config: config,
            buffer: Buffer::empty(),
            theme_manager: ThemeManager::new()
        }
    }

    pub fn initialize(&self) {
        let default_panic = panic::take_hook();

        panic::set_hook(Box::new(move |info| {
            // TODO: call self.quit()
            execute!(
                io::stdout(),
                cursor::Show,
                event::DisableBracketedPaste,
                event::DisableFocusChange,
                event::DisableMouseCapture,
                LeaveAlternateScreen
            ).unwrap();

            disable_raw_mode().unwrap();
            default_panic(info);
        }));

        enable_raw_mode().unwrap();
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            event::EnableBracketedPaste,
            event::EnableFocusChange,
            event::EnableMouseCapture,
            cursor::Hide
        ).unwrap();
    }

    pub fn quit(&self) {
        execute!(
            io::stdout(),
            cursor::Show,
            event::DisableBracketedPaste,
            event::DisableFocusChange,
            event::DisableMouseCapture,
            LeaveAlternateScreen
        ).unwrap();
        disable_raw_mode().unwrap();
        exit(0);
    }

    pub fn handle_events(&mut self) {
        match event::read().unwrap() {
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

    pub fn render(&self) {
        let theme = self.theme_manager.current_theme();

        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0),
            SetBackgroundColor(theme.background),
            SetForegroundColor(theme.foreground),
            Clear(ClearType::All),
            Print(format!("{}", self.buffer.text.to_string()))
        ).unwrap();
    }
}