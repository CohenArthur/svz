use std::{env, fs};

mod data_graph;
mod data_structures;
mod parser;
mod render;

use parser::Parser;
use render::Dot;

fn main() {
    // Read every file into a single string.
    // This is bad for memory AND performance, but it avoids switching the parser
    // to a streaming one, which brings its own set of trouble.
    // TODO: Improve that
    let input = env::args()
        .into_iter()
        .skip(1)
        .fold(String::new(), |acc, file| {
            format!("{}{}", acc, fs::read_to_string(file).unwrap())
        });

    println!("{}", Parser::parse(&input).to_dot());
}
