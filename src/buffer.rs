use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {

    pub fn load(file_path: &str) -> Result<Buffer, Error> {
        let file_contents = std::fs::read_to_string(file_path)?;
        let mut lines: Vec<String> = Vec::new();
        for line in file_contents.lines() {
            lines.push(line.to_string());
        }
        Ok(Self{lines})
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn get_line(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }

}
