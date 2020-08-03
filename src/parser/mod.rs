//! The parser module produces a data graph from a given input

use crate::data_graph::DataGraph;

pub struct Parser;

impl Parser {
    pub fn parse<'a>(data: &str) -> DataGraph<'a> {
        DataGraph::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::{DataStructure, DataField};

    fn assert_edge(dg: &DataGraph, lhs: &str, rhs: &str) {
        for (key, values) in dg.iter_all() {
            if key.name.as_ref().unwrap() == lhs {
                for value in values {
                    if value.name.as_ref().unwrap() == rhs {
                        assert!(true);
                    }
                }
            }
        }

        assert!(false);
    }

    fn get<'a>(dg: &'a DataGraph, name: &str) -> Option<&'a DataStructure> {
        for (key, _) in dg.iter_all() {
            if key.name.as_ref().unwrap() == name {
                return Some(key);
            }
        }

        None
    }

    #[test]
    fn basic_struct() {
        let input = r#"
        struct basic {
            size_t size;
            char[] buffer;
            void * ptr;
            void* ptr_2;
            struct basic **** multipointer;
        };
        "#;

        let dg = Parser::parse(input);

        let f0 = DataField::new("buffer".to_string(), "char[]".to_string());
        let f1 = DataField::new("size".to_string(), "size_t".to_string());
        let f2 = DataField::new("ptr".to_string(), "void *".to_string());
        let f3 = DataField::new("ptr_2".to_string(), "void*".to_string());

        assert_edge(&dg, "basic", "basic");

        assert!(get(&dg, "basic").unwrap().fields.contains(&f0));
        assert!(get(&dg, "basic").unwrap().fields.contains(&f1));
        assert!(get(&dg, "basic").unwrap().fields.contains(&f2));
        assert!(get(&dg, "basic").unwrap().fields.contains(&f3));
    }
}
