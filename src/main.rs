mod data_graph;
mod data_structures;
mod parser;
mod render;

use data_graph::DataGraph;
use data_structures::DataStructure;
use parser::Parser;
use render::Dot;

fn main() {
    let mut s_0 = DataStructure::new(Some("linked_list"));
    s_0.add_field("size", "size_t");
    s_0.add_field("capacity", "size_t");
    s_0.add_field("head", "struct node *");
    s_0.add_field("tail", "struct node *");

    let mut s_1 = DataStructure::new(Some("ll_node"));
    s_1.add_field("data", "void *");
    s_1.add_field("next", "struct node *");
    s_1.add_field("prev", "struct node *");

    let mut dg = DataGraph::new();

    dg.add_edge(&s_0, &s_1);
    dg.add_edge(&s_1, &s_1);

    println!("{}", dg.to_dot());
}
