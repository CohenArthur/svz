use std::{env, fs};

mod data_graph;
mod data_structures;
mod parser;
mod render;

use parser::Parser;

fn main() {
    for file in env::args().skip(1) {
        dbg!(&file);
        let input = fs::read_to_string(file).unwrap();

        println!("{}", Parser::parse(&input));
    }
}
