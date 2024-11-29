use std::collections::HashMap;

use crossterm::style::Color;

const fn from_rgb(rgb: u32) -> Color {
    let red = ((rgb >> 16) & 0xFF) as u8;
    let green = ((rgb >> 8) & 0xFF) as u8;
    let blue = (rgb & 0xFF) as u8;
    return Color::Rgb { r: red, g: green, b: blue }
}

pub struct ThemeManager {
    index: usize,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            index: THEMES.iter()
                .position(|value| *value == DEFAULT)
                .unwrap_or(0)
        }
    }

    pub fn current_theme(&self) -> &Theme {
        &THEMES[self.index]
    }

    pub fn next_theme(&mut self) {
        self.index = (self.index + 1) % THEMES.len();
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Light,
    Dark
}

#[derive(Debug, PartialEq)]
pub struct Theme<'a> {
    pub name: &'a str,
    pub mode: Mode,
    pub background: Color,
    pub foreground: Color,
    pub accent: Option<Color>
}

pub const DEFAULT: Theme = CATPPUCCIN_MACCHIATO;
pub const THEMES: &[Theme] = &[
    GRUVBOX_DARK,
    GRUVBOX_LIGHT,
    CATPPUCCIN_FRAPPE,
    CATPPUCCIN_MACCHIATO,
    CATPPUCCIN_MOCHA
];

pub const GRUVBOX_DARK: Theme = Theme {
    name: "Gruvbox (Dark)",
    mode: Mode::Dark,
    background: from_rgb(0x282828),
    foreground: from_rgb(0xEBDBB2),
    accent: Option::None
};

pub const GRUVBOX_LIGHT: Theme = Theme {
    name: "Gruvbox (Light)",
    mode: Mode::Light,
    background: from_rgb(0xFBF1C7),
    foreground: from_rgb(0x3C3836),
    accent: Option::None
};

pub const CATPPUCCIN_FRAPPE: Theme = Theme {
    name: "Catppuccin Frappé",
    mode: Mode::Dark,
    background: from_rgb(0x303446),
    foreground: from_rgb(0xC6D0F5),
    accent: Option::Some(from_rgb(0xCA9EE6))
};

pub const CATPPUCCIN_MACCHIATO: Theme = Theme {
    name: "Catppuccin Macchiato",
    mode: Mode::Dark,
    background: from_rgb(0x24273A),
    foreground: from_rgb(0xCAD3F5),
    accent: Option::Some(from_rgb(0xC6A0F6))
};

pub const CATPPUCCIN_MOCHA: Theme = Theme {
    name: "Catppuccin Mocha",
    mode: Mode::Dark,
    background: from_rgb(0x1E1E2E),
    foreground: from_rgb(0xCDD6F4),
    accent: Option::Some(from_rgb(0xCBA6F7))
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn unique_theme_colors() {
        let mut seen_names = HashSet::new();
        let mut seen_backgrounds = HashSet::new();
        let mut seen_foregrounds = HashSet::new();
        let mut seen_accents = HashSet::new();

        for theme in THEMES {
            assert!(seen_names.insert(theme.name), "Duplicate theme name found: {}", theme.name);
            assert!(seen_backgrounds.insert(theme.background), "Duplicate background color found for theme: {}", theme.name);
            assert!(seen_foregrounds.insert(theme.foreground), "Duplicate foreground color found for theme: {}", theme.name);
            assert!(seen_accents.insert(theme.accent), "Duplicate accent color found for theme: {}", theme.name);
        }
    }
}
