use crate::Row;

#[derive(Default, Debug)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open() -> Self {
        let mut rows = vec![Row::from("Foo bar")];
        rows.push(Row::from("Hello, world!!!!"));

        Self { rows }
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
