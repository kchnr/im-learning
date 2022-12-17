use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    //read the file blah.json
    let file = File::open("src/blah.json").unwrap();

    let p: Person = serde_json::from_reader(file).unwrap();

    println!("{:?}", p);
}
