use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::convert::TryFrom;

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Down,
    Left,
    Right,
}


// TODO: Implement long backspace
pub enum EditorCommand {
    Move(Direction),
    Insert(char),
    Backspace,
    Delete,
    Enter,
    Resize,
    Save,
    Quit,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match(code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                (KeyCode::Backspace, _) => Ok(Self::Backspace),
                (KeyCode::Delete, _) => Ok(Self::Delete),
                (KeyCode::Tab, _) => Ok(Self::Insert('\t')),
                (KeyCode::Enter, _) => Ok(Self::Enter),
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => Ok(Self::Save),
                (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Insert(character)),
                _ => Err(format!("Key Code not supported: {code:?}")),
            },
            Event::Resize(_, _) => Ok(Self::Resize),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}