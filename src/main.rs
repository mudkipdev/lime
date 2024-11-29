mod editor;
mod buffer;
mod ui;
mod config;
mod theme;

use editor::Editor;

fn main() {
    let mut editor = Editor::new();
    editor.initialize();

    loop {
        editor.render();
        editor.handle_events();
    }
}