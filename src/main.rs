mod data_structures;
mod render;

use data_structures::DataStructure;
use render::Dot;

fn main() {
    let mut stru = DataStructure::new(Some("SomeStruct".to_string()));

    stru.add_field("field0".to_string(), "char *".to_string());

    dbg!(stru.to_dot());
}
