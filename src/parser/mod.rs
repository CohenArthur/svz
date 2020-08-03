//! The parser module produces a data graph from a given input

use nom::{
    branch::alt, bytes::complete::is_not, bytes::complete::tag, bytes::complete::take_while1,
    character::complete::char, sequence::delimited, IResult,
};

use crate::data_graph::DataGraph;

pub struct Parser;

impl Parser {
    fn pointer(input: &str) -> IResult<&str, &str> {
        alt((
            take_while1(|c| c == '*'),
            delimited(char('['), is_not("]"), char(']')),
        ))(input)
    }

    fn space(input: &str) -> IResult<&str, &str> {
        take_while1(|c| c == ' ' || c == '\t')(input)
    }

    fn identifier(input: &str) -> IResult<&str, &str> {
        take_while1(|c| c != ' ' && c != '\t')(input)
    }

    fn typedef_tok(input: &str) -> IResult<&str, &str> {
        tag("typedef")(input)
    }

    fn struct_tok(input: &str) -> IResult<&str, &str> {
        tag("struct")(input)
    }

    pub fn parse<'a>(data: &str) -> DataGraph<'a> {
        DataGraph::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::{DataField, DataStructure};

    #[test]
    fn space() {
        assert_eq!(Parser::space("    "), Ok(("", "    ")));
        assert_eq!(Parser::space("  \t"), Ok(("", "  \t")));
        assert_eq!(Parser::space(" \tSomething"), Ok(("Something", " \t")));
        assert_eq!(Parser::space("   Something\t\t"), Ok(("Something\t\t", "   ")));
        assert_eq!(Parser::space("\t\t\t1234"), Ok(("1234", "\t\t\t")));
    }

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
