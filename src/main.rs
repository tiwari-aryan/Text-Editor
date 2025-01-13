#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]

pub mod editor;
pub mod terminal;
pub mod view;
pub mod buffer;
use editor::Editor;

fn main() { 
    Editor::new().unwrap().run();
}