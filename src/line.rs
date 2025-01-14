use std::{cmp::min, fmt, fmt::Display, ops::Range};

pub struct Line {
    pub string: String
}

impl Line {

    pub fn from(line: &str) -> Self {
        Self{
            string: String::from(line),
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = min(range.end, self.string.len());
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }

}

impl Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.string)
    }
}