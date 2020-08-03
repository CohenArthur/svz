//! The Dot trait is used when outputting a graph to the .dot format

pub trait Dot {
    fn to_dot(&self) -> String;
}
