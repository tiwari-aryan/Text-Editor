use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, size, EnterAlternateScreen, LeaveAlternateScreen, EnableLineWrap, DisableLineWrap, SetTitle};
use crossterm::{queue, Command};
use crossterm::cursor::{MoveTo, Show, Hide};
use crossterm::style::{Attribute, Print};
use std::io::{stdout, Write, Error};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Size {
    pub num_rows: usize,
    pub num_columns: usize, 
}

#[derive(Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Default for Size {
    fn default() -> Self {
        // Default terminal size for Windows Terminal
        Self{num_rows: 30, num_columns: 120}
    }
}

pub struct Terminal {
    
}



impl Terminal {
    
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::disable_line_wrap()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{row: 0, column: 0})?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        Self::disable_line_wrap()?;
        Self::leave_alternate_screen()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_command(EnableLineWrap)?;
        Ok(())
    }

    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_command(DisableLineWrap)?;
        Ok(())
    }

    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))?;
        Ok(())
    }

    pub fn reverse_colour() -> Result<(), Error> {
        Self::queue_command(Print(Attribute::Reverse))?;
        Ok(())
    }

    pub fn reset_colour() -> Result<(), Error> {
        Self::queue_command(Print(Attribute::Reset))?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.column as u16, position.row as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (num_columns, num_rows) = size()?;
        Ok(Size{num_rows: num_rows as usize, num_columns: num_columns as usize})
    }

    pub fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

}
