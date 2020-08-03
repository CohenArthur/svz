use std::{env, fs};

mod parser;
mod render;
mod data_structures;
mod data_graph;

use parser::Parser;

fn main() {
    for file in env::args() {
        let input = fs::read_to_string(file).unwrap();

        println!("{}", Parser::parse(&input));
    }
}
