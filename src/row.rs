use std::cmp;
use std::fmt::Display;

#[derive(Default, Clone)]
pub struct Row {
    content: String,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.content.len());
        let start = cmp::min(start, end);
        self.content[start..end].to_string()
    }
}

impl<T> From<T> for Row
where
    T: Display
{
    fn from(value: T) -> Self {
        Self {
            content: format!("{}", value),
        }
    }
}

impl Into<String> for Row {
    fn into(self) -> String {
        self.content
    }
}