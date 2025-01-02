use crate::terminal::{Terminal, Position, Size};
use crate::view::View;

use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::{env, io::Error};

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub x: u16,
    pub y: u16,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    update_cursor: bool,
    top_left: Location,
    location: Location,
    update_location: Location,
    view: View,
}

impl Editor {

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_path) = args.get(1) {
            self.view.load(file_path);
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_move_event(&mut self, code: &KeyCode){
        match code {
            KeyCode::Up if self.update_location.y > 0 => {
                self.update_location.y -= 1;
                self.update_cursor = true;
            },
            KeyCode::Down => {
                self.update_location.y += 1;
                self.update_cursor = true;
            },
            KeyCode::Left if self.update_location.x > 0 => {
                self.update_location.x -= 1;
                self.update_cursor = true;
            },
            KeyCode::Right => {
                self.update_location.x += 1;
                self.update_cursor = true;
            },
            _ => (),
        }
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right => {
                    self.evaluate_move_event(code);
                },
                _ => (),
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(Position{row: 0, column: 0})?;
            print!("Goodbye! \r\n");
        }
        else {
            let Size{num_rows, num_columns} = Terminal::size()?;
            self.view.render()?;
            if self.update_cursor {
                if self.update_location.x < self.top_left.x {
                    self.top_left.x = self.update_location.x;
                }
                if self.update_location.y < self.top_left.y {
                    self.top_left.y = self.update_location.y;
                }
                if self.update_location.x >= self.top_left.x + num_columns {
                    self.top_left.x = self.update_location.x - num_columns + 1;
                }
                if self.update_location.y >= self.top_left.y + num_rows {
                    self.top_left.y = self.update_location.y - num_rows + 1;
                }
                self.location = self.update_location;
                self.update_cursor = false;
            }
            Terminal::move_cursor_to(Position{row: self.update_location.y - self.top_left.y, column: self.update_location.x - self.top_left.x})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

}