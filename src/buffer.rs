use std::io::{Error, Write};
use std::fs::File;
use crate::editor::Location;
use crate::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub save_file_path: Option<String>,
    lines: Vec<Line>,
    pub is_modified: bool,
}

impl Buffer {

    pub fn load(file_path: &str) -> Result<Buffer, Error> {
        let file_contents = std::fs::read_to_string(file_path)?;
        let mut lines: Vec<Line> = Vec::new();
        for line in file_contents.lines() {
            lines.push(Line::from(line));
        }
        Ok(Self{save_file_path: Some(file_path.to_string()), lines, is_modified: false})
    }

    pub fn save_file(&self) -> Result<(), Error> {
        if let Some(file_name) = &self.save_file_path {
            let mut file = File::create(file_name)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn get_line(&self, index: usize) -> Option<&Line> {
        self.lines.get(index)
    }

    pub fn get_num_rows(&self) -> usize {
        self.lines.len()
    }

    pub fn get_num_columns(&self, index: usize) -> usize {
        if let Some(line) = self.get_line(index) {
            line.len()
        }
        else {
            0
        }
    }

    pub fn insert_character(&mut self, location: Location, character: char) {
        let Location{x, y} = location;
        if y as usize == self.lines.len() {
            self.lines.push(Line{string: character.to_string()});
        }
        else if let Some(line) = self.lines.get_mut(y as usize){
            line.string = format!("{}{}{}", line.get(0..x as usize), character, line.get(x as usize..line.len()));
        }
        else{
            panic!("Error: Could not add character.");
        }
        self.is_modified = true;
    }

    pub fn delete_character(&mut self, location: Location) {
        let Location{x, y} = location;
        if let Some(line) = self.lines.get(y as usize) {
            if (x as usize) == line.len() {
                self.lines[y as usize].string = format!("{}{}", line.get(0..x as usize), self.lines.remove((y + 1) as usize).string);
            }
            else {
                self.lines[y as usize].string = format!("{}{}", line.get(0..x as usize), line.get((x+1) as usize..line.len()));
            }
        }
        else {
            panic!("Error: Could not delete character.");
        }
        self.is_modified = true;
    }

    pub fn enter(&mut self, location: Location) {
        let Location{x, y} = location;
        if let Some(line) = self.lines.get(y as usize) {
            let current_line: Line = Line{string: line.get(0..(x as usize))};
            let next_line: Line = Line{string: line.get((x as usize)..line.len())};

            self.lines[y as usize].string = current_line.string;
            self.lines.insert((y + 1) as usize, next_line);
        }
        else if (y as usize) == self.get_num_rows() {
            self.lines.push(Line{string: String::from("")});
        }
        self.is_modified = true;
    }

}