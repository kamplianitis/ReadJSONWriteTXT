use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
struct MyData {
    dicts: Vec<serde_json::Value>,
}

fn main() {
    let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap();

    let mut contents = String::new();

    let mut file = File::open(input_path.clone()).expect("Failed to open");

    let mut outputfile = File::create(output_path).expect("Failed to create file");

    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let data: MyData = serde_json::from_str(&contents).expect("Failed to parse");

    writeln!(outputfile, "{}", std::env::args().nth(3).unwrap()).expect("Failed to write");

    for dict in data.dicts {
        if let Some(fields) = dict.get("fields") {
            if let Some(value) = fields.get("value") {
                if let Some(name_str) = value.as_str() {
                    if let Some(catgory) = fields.get("framework") {
                        if let Some(catgory_str) = catgory.as_str() {
                            writeln!(outputfile, "{}, {}", name_str, catgory_str)
                                .expect("File not found");
                        }
                    }
                }
            }
        }
    }
}
