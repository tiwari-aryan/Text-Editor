use crate::editorcommand::EditorCommand;
use crate::terminal::{Terminal, Size};
use crate::view::View;
use crate::statusbar::StatusBar;
use crate::messagebar::MessageBar;
use crate::uicomponent::UIComponent;

use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use std::{cmp, env, io::Error, panic::{set_hook, take_hook}};

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

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    size: Size,
}

impl Editor {

    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut editor = Self::default();
        editor.size = Terminal::size().unwrap_or_default();
        editor.resize(editor.size);

        let args: Vec<String> = env::args().collect();
        if let Some(file_path) = args.get(1) {
            editor.view.load(file_path);
            Terminal::set_title(&file_path)?;
        }

        editor.message_bar.set_message("HELP: Ctrl-S = save | Ctrl-Q = quit".to_string());
        Ok(editor)
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

    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.view.resize(Size{num_rows: size.num_rows - 2, num_columns: size.num_columns});
        self.status_bar.resize(Size{num_rows: 1, num_columns: size.num_columns});
        self.message_bar.resize(Size{num_rows: 1, num_columns: size.num_columns});
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
                else if let EditorCommand::Resize(size) = command {
                    self.resize(size);
                    self.view.set_redraw(true);
                }
                else {
                    self.view.handle_command(command);
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        if self.size.num_rows == 0 || self.size.num_columns == 0 {
            return;
        }
        let _ = Terminal::hide_cursor();
        self.status_bar.set_status(self.view.get_status());
        if self.size.num_rows > 0 {
            self.message_bar.render(self.size.num_rows - 1);
        }
        if self.size.num_rows > 1 {
            self.status_bar.render(self.size.num_rows - 2);
        }
        if self.size.num_rows > 2 {
            self.view.render(0);
        }
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