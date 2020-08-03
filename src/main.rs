mod data_graph;
mod data_structures;
mod render;

use data_graph::DataGraph;
use data_structures::DataStructure;
use render::Dot;

fn main() {
    let mut s_0 = DataStructure::new(Some("linked_list".to_string()));
    s_0.add_field("size".to_string(), "size_t".to_string());
    s_0.add_field("capacity".to_string(), "size_t".to_string());
    s_0.add_field("head".to_string(), "struct node *".to_string());
    s_0.add_field("tail".to_string(), "struct node *".to_string());

    let mut s_1 = DataStructure::new(Some("ll_node".to_string()));
    s_1.add_field("data".to_string(), "void *".to_string());
    s_1.add_field("next".to_string(), "struct node *".to_string());
    s_1.add_field("prev".to_string(), "struct node *".to_string());

    let mut dg = DataGraph::new();

    dg.add_edge(&s_0, &s_1);

    println!("{}", dg.to_dot());
}
