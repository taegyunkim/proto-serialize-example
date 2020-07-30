extern crate handlebars;
extern crate protobuf;
extern crate serde;

use handlebars::Handlebars;
use protobuf::Message;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod node;

#[derive(Serialize)]
struct Data {
    length: usize,
    bytes: Vec<u8>,
}

fn main() {
    let node = create_node(String::from("abc"));

    let bytes = node.write_to_bytes().unwrap();
    println!("{:?}", bytes);

    let template_path = Path::new("test.cc.handlebars");
    let display = template_path.display();
    let mut template_file = match File::open(&template_path) {
        Err(msg) => panic!("Failed to open {}: {}", display, msg),
        Ok(file) => file,
    };

    let mut template_str = String::new();
    if let Err(msg) = template_file.read_to_string(&mut template_str) {
        panic!("Failed to read {}: {}", display, msg)
    }

    let handlebars = Handlebars::new();
    let data = Data {
        length: bytes.len(),
        bytes,
    };

    let output = handlebars
        .render_template(&template_str, &data)
        .expect("handlebar render failed");
    let mut file = File::create("test.cc").expect("file create failed");
    file.write_all(output.as_bytes()).expect("write failed");
}

pub fn create_node(id: String) -> node::Node {
    node::Node {
        id,
        ..Default::default()
    }
}
