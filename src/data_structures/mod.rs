//! This module exposes some traits used when representing data structures

use crate::render;

const BASE_PADDING: usize = 4;

/// Fields contained inside the data structures
#[derive(Debug)]
struct DataField {
    name: String,
    type_name: String,
}

/// Struct used to represent the different data structures that svz will parse
#[derive(Debug)]
pub struct DataStructure {
    name: Option<String>,
    fields: Vec<DataField>,
    padding: usize,
}

impl DataStructure {
    /// Create a new data structure with a given name
    pub fn new(name: Option<String>) -> DataStructure {
        DataStructure {
            name,
            fields: vec![],
            padding: BASE_PADDING
        }
    }

    /// Inserts a field into the data structure
    pub fn add_field(&mut self, name: String, type_name: String) {
        // Set the padding as the size of the longest type + BASE_PADDING
        let tmp_padding = type_name.len() + BASE_PADDING;
        self.padding = if tmp_padding > self.padding { tmp_padding } else { self.padding };
        self.fields.push(DataField { name, type_name })
    }
}

impl render::Dot for DataStructure {
    fn to_dot(&self) -> String {
        let mut base = format!("{} [label=<<B>struct {}</B>", self.name.as_ref().unwrap(), self.name.as_ref().unwrap()); // FIXME: Dont' unwrap
        for field in self.fields.iter() {
            // Newline + align left
            base.push_str("<BR ALIGN=\"LEFT\"/>");
            base.push_str(&field.type_name);

            // Add padding spaces
            for _ in 0..self.padding - field.type_name.len() { base.push(' '); }

            base.push_str(&field.name);
        }

        // Close the HTML-like string
        base.push_str("<BR ALIGN=\"LEFT\"/>>]");

        base
    }
}
