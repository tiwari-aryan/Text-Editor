use crate::terminal::{Terminal, Position, Size};
use crate::view::View;

use crossterm::event::{read, Event, Event::{Key, Resize}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::{env, io::Error, panic::{set_hook, take_hook}};

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub x: u16,
    pub y: u16,
}

pub struct Editor {
    should_quit: bool,
    update_cursor: bool,
    top_left: Location,
    location: Location,
    update_location: Location,
    view: View,
}

impl Editor {

    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_path) = args.get(1) {
            view.load(file_path);
        }
        Ok(Self{
            should_quit: false,
            update_cursor: false,
            top_left: Location::default(),
            location: Location::default(),
            update_location: Location::default(),
            view})
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Error: Could not read event {err:?}")
                    }
                }
            }
        }
    }

    fn evaluate_move_event(&mut self, code: KeyCode) {
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

    fn evaluate_event(&mut self, event: Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event {
            match code {
                KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => {
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
        else if let Resize(..) = event {
            self.view.needs_redraw = true;
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        let Size{num_rows, num_columns} = Terminal::size().unwrap_or_default();
        self.view.render(self.top_left);
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
            self.view.needs_redraw = true;
        }
        let _ = Terminal::move_cursor_to(Position{row: self.update_location.y - self.top_left.y, column: self.update_location.x - self.top_left.x});
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }

}

impl Drop for Editor {
    fn drop(&mut self){
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye!\r\n");
        }
    }
}