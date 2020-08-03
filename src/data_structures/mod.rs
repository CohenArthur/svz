//! This module exposes some traits used when representing data structures

use crate::render;

/// Default padding between the field's type and its name
const BASE_PADDING: usize = 4;

/// Accent color used when displaying types
const ACCENT_COLOR: &str = "purple";

/// Fields contained inside the data structures
// FIXME: Do not copy ?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DataField<'a> {
    name: &'a str,
    type_name: &'a str,
}

impl <'a> DataField<'a> {
    // Create a new DataField
    pub fn new(type_name: &'a str, name: &'a str) -> DataField<'a> {
        DataField { name, type_name }
    }
}

/// Struct used to represent the different data structures that svz will parse
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DataStructure<'a> {
    pub name: Option<&'a str>,

    #[cfg(test)]
    pub fields: Vec<DataField<'a>>,

    #[cfg(not(test))]
    fields: Vec<DataField<'a>>,

    padding: usize,
}

impl <'a> DataStructure<'a> {
    /// Create a new data structure with a given name
    pub fn new(name: Option<&'a str>) -> DataStructure<'a> {
        DataStructure {
            name,
            fields: vec![],
            padding: BASE_PADDING,
        }
    }

    /// Inserts a field into the data structure
    pub fn add_field(&mut self, field: DataField<'a>) {
        // Set the padding as the size of the longest type + BASE_PADDING
        self.padding = std::cmp::max(field.type_name.len() + BASE_PADDING, self.padding);
        self.fields.push(field)
    }
}

impl render::Dot for DataStructure<'_> {
    fn to_dot(&self) -> String {
        let mut base = format!("{0} [label=<<B>struct {0}</B>", self.name.as_ref().unwrap(),); // FIXME: Dont' unwrap
        for field in self.fields.iter() {
            // Newline + align left
            base.push_str("<BR ALIGN=\"LEFT\"/>");
            base.push_str(&format!(
                "<FONT COLOR=\"{}\">{}</FONT>",
                ACCENT_COLOR, field.type_name
            ));

            // Padding + field name
            base.push_str(&format!(
                "{: <1$}{2}",
                "",
                self.padding - field.type_name.len(),
                field.name
            ));
        }

        // Close the HTML-like string
        base.push_str("<BR ALIGN=\"LEFT\"/>>]");

        base
    }
}
