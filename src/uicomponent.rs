use std::io::Error;
use crate::terminal::Size;

pub trait UIComponent {

    fn set_redraw(&mut self, needs_redraw: bool);

    fn needs_redraw(&self) -> bool;

    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_redraw(true);
    }

    fn set_size(&mut self, size: Size);

    fn render(&mut self, start_row: usize) {
        if self.needs_redraw() {
            match self.draw(start_row) {
                Ok(()) => self.set_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not render component: {err:?}");
                    }
                }
            }
        }
        self.set_redraw(false);
    }

    fn draw(&mut self, start_row: usize) -> Result<(), Error>;

}