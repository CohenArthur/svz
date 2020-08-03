//! This module exposes some traits used when representing data structures

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
        self.fields.push(DataField { name, type_name })
    }
}
