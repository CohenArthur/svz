mod data_structures;
use data_structures::DataStructure;

fn main() {
    let mut stru = DataStructure::new(Some("SomeStruct".to_string()));

    stru.add_field("field0".to_string(), "char *".to_string());

    dbg!(stru);
}
