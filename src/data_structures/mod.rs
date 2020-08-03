//! This module exposes some traits used when representing data structures

use crate::render;

/// Fields contained inside the data structures
#[derive(Debug)]
struct DataField {
    name: String,
    type_name: String,
    padding: String,
}

/// Struct used to represent the different data structures that svz will parse
#[derive(Debug)]
pub struct DataStructure {
    name: Option<String>,
    fields: Vec<DataField>,
}

impl DataStructure {
    /// Create a new data structure with a given name
    pub fn new(name: Option<String>) -> DataStructure {
        DataStructure {
            name,
            fields: vec![],
        }
    }

    /// Inserts a field into the data structure
    pub fn add_field(&mut self, name: String, type_name: String) {
        self.fields.push(DataField { name, type_name, padding: "    ".to_string()})
    }
}

impl render::Dot for DataField {
    fn to_dot(&self) -> String {
        format!("{}{}{}", self.type_name, self.padding, self.name)
    }
}

impl render::Dot for DataStructure {
    fn to_dot(&self) -> String {
        let mut base = format!("struct {}\n", self.name.as_ref().unwrap()); // FIXME: Dont' unwrap
        for field in self.fields.iter() {
            base.push_str(&field.to_dot());
        }

        base
    }
}
