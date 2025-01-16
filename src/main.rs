#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]

pub mod editor;
pub mod editorcommand;
pub mod terminal;
pub mod view;
pub mod statusbar;
pub mod messagebar;
pub mod buffer;
pub mod line;
pub mod uicomponent;
use editor::Editor;

fn main() { 
    Editor::new().unwrap().run();
}