//! The parser module produces a data graph from a given input

use crate::data_graph::DataGraph;

pub struct Parser {
}

impl Parser {
    pub fn parse<'a>() -> DataGraph<'a> {
        DataGraph::new()
    }
}
