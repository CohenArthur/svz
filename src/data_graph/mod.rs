//! A data graph is simply a graph where every struct is linked to the other
//! structs it references
//!
//! For example, in the following example:
//! ```c
//! struct node {
//!     void *data;
//!
//!     struct node *next;
//!     struct node *prev;
//! }
//!
//! struct d_linked_list {
//!     size_t size;
//!     size_t capacity;
//!
//!     struct node *head;
//!     struct node *tail;
//! }
//! ```
//!
//! the `d_linked_list` structure is linked to the `node` one, which is linked
//! to itself.
//!
//! It's basically just a multimap with a few methods

use multimap::MultiMap;

use crate::data_structures::DataStructure;
use crate::render::Dot;

/// Basic multimap
pub struct DataGraph<'a> {
    data: MultiMap<&'a DataStructure, &'a DataStructure>,
}

impl<'a> DataGraph<'a> {
    /// Create a new empty struct graph
    pub fn new() -> DataGraph<'a> {
        DataGraph {
            data: MultiMap::new(),
        }
    }

    /// Add a "link" between two structs
    pub fn add_edge(&mut self, lhs: &'a DataStructure, rhs: &'a DataStructure) {
        self.data.insert(lhs, rhs)
    }

    pub fn iter_all(&self) -> multimap::IterAll<'_, &DataStructure, std::vec::Vec<&DataStructure>> {
        self.data.iter_all()
    }
}

impl Dot for DataGraph<'_> {
    fn to_dot(&self) -> String {
        // Graphviz header
        let mut base = String::from("digraph svz {");

        for (key, values) in self.data.iter_all() {
            base.push_str(&format!("{}\n", key.to_dot()));

            // Add each dependency
            for value in values.iter() {
                base.push_str(&format!("{}\n", value.to_dot()));

                // Add the edge
                base.push_str(&format!(
                    "{} -> {};\n",
                    key.name.as_ref().unwrap(),
                    value.name.as_ref().unwrap()
                )); // FIXME: NO unwrap
            }
        }

        // Closing bracket from the header
        base.push('}');
        base
    }
}
