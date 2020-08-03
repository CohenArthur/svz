//! The parser module produces a data graph from a given input

use nom::{
    branch::alt, bytes::complete::is_a, bytes::complete::is_not, bytes::complete::tag,
    bytes::complete::take, bytes::complete::take_until, bytes::complete::take_while,
    bytes::complete::take_while1, character::complete::char, character::is_alphanumeric,
    combinator::opt,
    character::is_space, sequence::delimited, IResult,
};

use crate::data_graph::DataGraph;
use crate::data_structures::{DataField, DataStructure};

pub struct Parser;

impl Parser {
    fn array(input: &str) -> IResult<&str, &str> {
        let (input, _) = char('[')(input)?;
        let (input, _) = take_until("]")(input)?;
        let (input, _) = char(']')(input)?;
        let (input, _) = take_while(|c| is_space(c as u8))(input)?;

        // FIXME: Get actual value contained between brackets. Maybe recognize() ?
        Ok((input, "[]"))
    }

    fn asterisks(input: &str) -> IResult<&str, &str> {
        take_while1(|c| c == '*' || is_space(c as u8))(input)
    }

    fn pointer(input: &str) -> IResult<&str, &str> {
        alt((Parser::asterisks, Parser::array))(input)
    }

    fn space(input: &str) -> IResult<&str, &str> {
        take_while1(|c| is_space(c as u8))(input)
    }

    fn identifier(input: &str) -> IResult<&str, &str> {
        take_while1(|c| !is_space(c as u8))(input)
    }

    fn tok<'a>(input: &'a str, tok: &'a str) -> IResult<&'a str, &'a str> {
        tag(tok)(input)
    }

    fn typedef_tok(input: &str) -> IResult<&str, &str> {
        Parser::tok(input, "typedef")
    }

    fn struct_tok(input: &str) -> IResult<&str, &str> {
        Parser::tok(input, "struct")
    }

    fn enum_tok(input: &str) -> IResult<&str, &str> {
        Parser::tok(input, "enum")
    }

    fn union_tok(input: &str) -> IResult<&str, &str> {
        Parser::tok(input, "union")
    }

    fn parse_type(input: &str) -> IResult<&str, &str> {
        Ok(("", ""))
    }

    fn parse_field(input: &str) -> IResult<&str, DataField> {
        let (input, type_name) = Parser::identifier(input)?;
        let (input, _) = Parser::space(input)?;
        let (input, name) = Parser::identifier(input)?;

        Ok((input, DataField::new(type_name, name)))
    }

    fn parse_struct(input: &str) -> IResult<&str, DataStructure> {
        Ok((input, DataStructure::new(Some("test"))))
    }

    pub fn parse<'a>(data: &str) -> DataGraph<'a> {
        DataGraph::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space() {
        assert_eq!(Parser::space("    "), Ok(("", "    ")));
        assert_eq!(Parser::space("  \t"), Ok(("", "  \t")));
        assert_eq!(Parser::space(" \tSomething"), Ok(("Something", " \t")));
        assert_eq!(
            Parser::space("   Something\t\t"),
            Ok(("Something\t\t", "   "))
        );
        assert_eq!(Parser::space("\t\t\t1234"), Ok(("1234", "\t\t\t")));
    }

    #[test]
    fn struct_tok() {
        assert_eq!(Parser::struct_tok("struct"), Ok(("", "struct")));
        assert_eq!(Parser::struct_tok("struct name"), Ok((" name", "struct")));
        assert_eq!(
            Parser::struct_tok("struct name { int f0; char f1; }"),
            Ok((" name { int f0; char f1; }", "struct"))
        );
    }

    #[test]
    fn typedef_tok() {
        assert_eq!(Parser::typedef_tok("typedef"), Ok(("", "typedef")));
        assert_eq!(
            Parser::typedef_tok("typedef struct"),
            Ok((" struct", "typedef"))
        );
    }

    #[test]
    fn typedef_struct() {
        match Parser::typedef_tok("typedef struct name { type field0; type field1; }") {
            Ok((remaining, _)) => {
                let remaining = Parser::space(remaining).unwrap().0;
                assert_eq!(
                    Parser::struct_tok(remaining),
                    Ok((" name { type field0; type field1; }", "struct"))
                );
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn asterisks() {
        assert_eq!(Parser::asterisks("*"), Ok(("", "*")));
        assert_eq!(Parser::asterisks("*****"), Ok(("", "*****")));
        assert_eq!(Parser::asterisks("** *"), Ok(("", "** *")));
    }

    #[test]
    fn array() {
        assert_eq!(Parser::array("[]"), Ok(("", "[]")));
        assert_eq!(Parser::array("[12]"), Ok(("", "[]")));
        assert_eq!(Parser::array("[MACRO]"), Ok(("", "[]")));
        assert_eq!(Parser::array("[MACRO + MACRO + 12]"), Ok(("", "[]")));
        assert_eq!(Parser::array("[MACRO] rest"), Ok(("rest", "[]")));
    }

    #[test]
    fn pointer() {
        assert_eq!(Parser::pointer("*"), Ok(("", "*")));
        assert_eq!(Parser::pointer("[]"), Ok(("", "[]")));
        assert_eq!(Parser::pointer("[] rest"), Ok(("rest", "[]")));
        assert_eq!(Parser::pointer("** rest"), Ok(("rest", "** ")));
    }

    #[test]
    fn identifier() {
        assert_eq!(Parser::identifier("size_t id"), Ok((" id", "size_t")));
        assert_eq!(Parser::identifier("1255 something"), Ok((" something", "1255")));
    }

    #[test]
    fn parse_field() {
        assert_eq!(Parser::parse_field("size_t something"), Ok(("", DataField::new("size_t", "something"))));
        assert_eq!(Parser::parse_field("char* buffer"), Ok(("", DataField::new("char*", "buffer"))));
        assert_eq!(Parser::parse_field("char **** buffer"), Ok(("", DataField::new("char ****", "buffer"))));
        assert_eq!(Parser::parse_field("char[] buffer"), Ok(("", DataField::new("char[]", "buffer"))));
        assert_eq!(Parser::parse_field("struct somestruct * ptr"), Ok(("", DataField::new("struct somestruct *", "ptr"))));
    }

    fn assert_edge(dg: &DataGraph, lhs: &str, rhs: &str) {
        for (key, values) in dg.iter_all() {
            if key.name.unwrap() == lhs {
                for value in values {
                    if value.name.unwrap() == rhs {
                        assert!(true);
                    }
                }
            }
        }

        assert!(false);
    }

    fn get<'a>(dg: &'a DataGraph, name: &str) -> Option<&'a DataStructure<'a>> {
        for (key, _) in dg.iter_all() {
            if key.name.unwrap() == name {
                return Some(key);
            }
        }

        None
    }

    /*
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
    */
}
