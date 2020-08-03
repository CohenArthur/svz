mod data_graph;
mod data_structures;
mod render;

use data_graph::DataGraph;
use data_structures::DataStructure;
use render::Dot;

fn main() {
    let mut s_0 = DataStructure::new(Some("SomeStruct".to_string()));

    s_0.add_field("buffer".to_string(), "char *".to_string());
    s_0.add_field("size".to_string(), "size_t".to_string());
    s_0.add_field("capacity".to_string(), "size_t".to_string());
    s_0.add_field(
        "field0".to_string(),
        "struct SomeVeryLongStruct".to_string(),
    );

    let mut s_1 = DataStructure::new(Some("AnotherStruct".to_string()));
    s_1.add_field("ptr".to_string(), "struct SomeStruct*".to_string());

    let mut dg = DataGraph::new();

    dg.add_edge(&s_0, &s_1);

    println!("{}", dg.to_dot());
}
