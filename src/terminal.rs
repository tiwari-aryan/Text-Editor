use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, size};
use crossterm::{queue, Command};
use crossterm::cursor::{MoveTo, Show, Hide};
use crossterm::style::Print;
use std::io::{stdout, Write, Error};

#[derive(Copy, Clone)]
pub struct Size {
    pub num_rows: u16,
    pub num_columns: u16, 
}

#[derive(Copy, Clone)]
pub struct Position {
    pub row: u16,
    pub column: u16,
}

pub struct Terminal {
    
}

impl Terminal {

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{row: 0, column: 0})?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
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
        Self::queue_command(MoveTo(position.column, position.row))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (num_columns, num_rows) = size()?;
        Ok(Size{num_rows, num_columns})
    }

    pub fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

}
