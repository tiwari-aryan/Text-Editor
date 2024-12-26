#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]

pub mod editor;
pub mod terminal;
use editor::Editor;

fn main() { 
    Editor::default().run();
}