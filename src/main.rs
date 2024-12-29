#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]

pub mod editor;
pub mod terminal;
pub mod view;
use editor::Editor;

fn main() { 
    Editor::default().run();
}