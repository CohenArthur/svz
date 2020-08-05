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

use std::collections::{HashMap, HashSet};

use crate::data_structures::DataStructure;
use crate::render::Dot;

// Define helper types cause they're a mouthful
type Key<'k> = &'k DataStructure<'k>;
type Value<'v> = HashSet<&'v DataStructure<'v>>;

/// Basic multimap
pub struct DataGraph<'a> {
    data: HashMap<Key<'a>, Value<'a>>,
}

impl<'a> DataGraph<'a> {
    /// Create a new empty struct graph
    pub fn new() -> DataGraph<'a> {
        DataGraph {
            data: HashMap::new(),
        }
    }

    /// Add a node without any edges
    pub fn add_node(&mut self, node: Key<'a>) -> Option<Value> {
        // Do not "erase" the existing edges in case they exit already
        match self.data.contains_key(node) {
            // Insert a new key with no edges
            false => self.data.insert(node, HashSet::new()),
            true => None,
        }
    }

    /// Add a "link" between two structs
    pub fn add_edge(&mut self, lhs: Key<'a>, rhs: Key<'a>) {
        self.add_node(lhs);
        self.add_node(rhs);

        // We can unwrap since we KNOW the key alreay exists.
        self.data.get_mut(lhs).unwrap().insert(rhs);
    }
}

impl Dot for DataGraph<'_> {
    fn to_dot(&self) -> String {
        // Graphviz header
        let mut base = String::from("digraph svz {\n");

        for (key, values) in self.data.iter() {
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

        // Closing bracket from the header
        base.push('}');
        base
    }
}
