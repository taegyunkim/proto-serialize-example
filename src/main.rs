extern crate protobuf;

use protobuf::Message;

pub mod node;

fn main() {
    let node = create_node(String::from("abc"));

    let data = node.write_to_bytes().unwrap();
    println!("{:?}", data);
}

pub fn create_node(id: String) -> node::Node {
    node::Node {
        id,
        ..Default::default()
    }
}
