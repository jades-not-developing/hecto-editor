use crate::Row;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let source = fs::read_to_string(path)?;
        let rows = source.lines().map(Row::from).collect();
        Ok(Self { rows })
    }

    pub fn row(&self, index: usize) -> Option<Row> {
        self.rows.get(index).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}