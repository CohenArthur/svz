use std::{env, fs};

mod data_graph;
mod data_structures;
mod parser;
mod render;

use data_graph::DataGraph;
use parser::Parser;
use render::Dot;

fn main() {
    // Panic if there is not enough arguments
    let file = env::args().nth(1).unwrap();

    let input = fs::read_to_string(file).unwrap();

    println!("{}", Parser::parse(&input).to_dot());
}
