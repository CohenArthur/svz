//! A data graph is simply a graph where every struct is linked to the other
//! structs it references
//!
//! For example, in the following example:
//! ```c
//! struct ll_node {
//!     void *data;
//!
//!     struct ll_node *next;
//!     struct ll_node *prev;
//! }
//!
//! struct d_linked_list {
//!     size_t size;
//!     size_t capacity;
//!
//!     struct ll_node *head;
//!     struct ll_node *tail;
//! }
//! ```
//!
//! the `d_linked_list` structure is linked to the `ll_node` one, which is linked
//! to itself.
//!
//! It's basically just a multimap with a few methods

use petgraph::{graph::NodeIndex, Graph};

use crate::data_structures::DataStructure;
use crate::render::Dot;

/// Basic multimap
pub struct DataGraph<'a> {
    data: Graph<DataStructure<'a>, ()>,
}

impl<'a> DataGraph<'a> {
    /// Create a new empty struct graph
    pub fn new() -> DataGraph<'a> {
        DataGraph { data: Graph::new() }
    }

    /// Add a node without any edges
    pub fn add_node(&mut self, node: DataStructure<'a>) {
        self.data.add_node(node);

        for s_idx in self.data.node_indices() {
            for d_idx in self.data.node_indices() {
                // FIXME: Don't unwrap
                if self.data[s_idx].fields_contain(self.data[d_idx].name().unwrap()) {
                    self.data.add_edge(s_idx, d_idx, ());
                }
            }
        }
    }
}

impl Dot for DataGraph<'_> {
    fn to_dot(&self) -> String {
        // Graphviz header
        let mut base = String::from("digraph svz {\n");

        for source in self.data.node_indices() {
            base.push_str(&format!("{}\n", self.data[source].to_dot()));
            for dest in self.data.neighbors(source) {
                base.push_str(&format!(
                    "{} -> {};\n",
                    self.data[source].name().as_ref().unwrap(),
                    self.data[dest].name().as_ref().unwrap(),
                ));
            }
        }

        /*
        for (key, values) in self.data. {
            base.push_str(&format!("{}\n", key.to_dot()));

            // Add each dependency
            for value in values.iter() {
                // Add the edge
                base.push_str(&format!(
                    "{} -> {};\n",
                    key.name().as_ref().unwrap(),
                    value.name().as_ref().unwrap()
                )); // FIXME: No unwrap
            }
        }
        */

        // Closing bracket from the header
        base.push('}');
        base
    }
}
