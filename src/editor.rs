use crate::editorcommand::EditorCommand;
use crate::terminal::Terminal;
use crate::view::View;
use crate::statusbar::StatusBar;

use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use std::{env, io::Error, panic::{set_hook, take_hook}};

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Eq, PartialEq)]
pub struct DocumentStatus {
    pub file_path: Option<String>,
    pub current_line: usize,
    pub total_lines: usize,
    pub is_modified: bool,
}

pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
}

impl Editor {

    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::new(2);
        let args: Vec<String> = env::args().collect();
        if let Some(file_path) = args.get(1) {
            view.load(file_path);
        }
        Ok(Self{
            should_quit: false,
            view,
            status_bar: StatusBar::default(),
        })
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

    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent{kind, ..}) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };
        if should_process {
            if let Ok(command) = EditorCommand::try_from(event) {
                if matches!(command, EditorCommand::Quit) {
                    self.should_quit = true;
                }
                else if matches!(command, EditorCommand::Resize) {
                    self.view.needs_redraw = true;
                    self.status_bar.needs_redraw = true;
                }
                else {
                    self.view.handle_command(command);
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.status_bar.set_status(self.view.get_status());
        self.view.render();
        self.status_bar.render();
        let _ = Terminal::move_cursor_to(self.view.get_cursor_position());
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