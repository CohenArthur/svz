mod data_structures;
mod render;

use data_structures::DataStructure;
use render::Dot;

fn main() {
    println!("digraph svz {{");

    let mut stru = DataStructure::new(Some("SomeStruct".to_string()));

    stru.add_field("buffer".to_string(), "char *".to_string());
    stru.add_field("size".to_string(), "size_t".to_string());
    stru.add_field("capacity".to_string(), "size_t".to_string());
    stru.add_field("field0".to_string(), "struct SomeVeryLongStruct".to_string());

    println!("{}", stru.to_dot());
    println!("SomeStruct -> \"a\"");
    println!("}}");
}
